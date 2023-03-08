use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use rppal::gpio::{Gpio, Level, Mode};
use thiserror::Error as LibError;

enum PwmUpdate {
    Stop,
    Frequency(f32),
    DutyCycle(f32),
}

#[derive(Debug, LibError)]
pub enum Error {
    #[error("PWM update error")]
    UpdateError,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct SoftPwm {
    channel: mpsc::Sender<PwmUpdate>,
    worker: Option<JoinHandle<()>>,
}

impl SoftPwm {
    pub fn new(gpio: Arc<Mutex<Gpio>>, pin: u8, frequency: f32, duty_cycle: f32) -> SoftPwm {
        {
            let mut g = gpio.lock().unwrap();
            g.set_mode(pin, Mode::Output);
            g.write(pin, Level::Low);
        }

        let (tx, rx) = mpsc::channel();

        SoftPwm {
            channel: tx,
            worker: Some(thread::spawn(move || {
                let mut worker = SoftPwmWorker::new(gpio, pin, frequency, duty_cycle, rx);

                worker.run();
            })),
        }
    }

    pub fn set_frequency(&mut self, new_frequency: f32) -> Result<()> {
        self.channel
            .send(PwmUpdate::Frequency(new_frequency))
            .map_err(|_| Error::UpdateError)
    }

    pub fn set_duty_cycle(&mut self, new_duty_cycle: f32) -> Result<()> {
        self.channel
            .send(PwmUpdate::DutyCycle(new_duty_cycle))
            .map_err(|_| Error::UpdateError)
    }
}

struct SoftPwmWorker {
    gpio: Arc<Mutex<Gpio>>,
    pin: u8,
    frequency: f32,
    duty_cycle: f32,
    channel: mpsc::Receiver<PwmUpdate>,
    time_on: Duration,
    time_off: Duration,
}

impl SoftPwmWorker {
    fn new(
        gpio: Arc<Mutex<Gpio>>,
        pin: u8,
        init_frequency: f32,
        init_duty_cycle: f32,
        channel: mpsc::Receiver<PwmUpdate>,
    ) -> SoftPwmWorker {
        SoftPwmWorker {
            gpio,
            pin,
            frequency: init_frequency,
            duty_cycle: init_duty_cycle,
            channel,
            time_on: Duration::ZERO,
            time_off: Duration::ZERO,
        }
    }

    fn update_times(&mut self) {
        let period_sec = 1.0 / self.frequency;
        let time_on_sec = period_sec * self.duty_cycle;
        let time_off_sec = period_sec - time_on_sec;
        self.time_on = Duration::from_nanos((time_on_sec * 1000000000.0) as u64);
        self.time_off = Duration::from_nanos((time_off_sec * 1000000000.0) as u64);
    }

    fn check_updates(&mut self, timeout: Duration) -> Option<(Duration, Duration)> {
        let mut updated = false;
        if let Ok(update) = self.channel.recv_timeout(timeout) {
            match update {
                PwmUpdate::Stop => return None,
                PwmUpdate::Frequency(nf) => {
                    self.frequency = nf;
                    updated = true;
                }
                PwmUpdate::DutyCycle(ndc) => {
                    self.duty_cycle = ndc;
                    updated = true;
                }
            }
        }

        if updated {
            self.update_times();
        }

        Some((self.time_on, self.time_off))
    }

    fn drive(&mut self, duration: Duration, level: Level) {
        if !duration.is_zero() {
            let gpio = self.gpio.lock().unwrap();
            gpio.write(self.pin, level);
            drop(gpio);
            thread::sleep(duration);
        }
    }

    fn run(&mut self) {
        loop {
            if let Some((time_on, _)) = self.check_updates(self.time_on) {
                //                println!("Pin {} HIGH for {} ns.", self.pin, time_on);
                self.drive(time_on, Level::High);
            } else {
                break;
            }

            if let Some((_, time_off)) = self.check_updates(self.time_off) {
                //                println!("Pin {} LOW for {} ns.", self.pin, time_off);
                self.drive(time_off, Level::Low);
            } else {
                break;
            }
        }
    }
}

impl Drop for SoftPwm {
    fn drop(&mut self) {
        if let Some(handle) = self.worker.take() {
            self.channel
                .send(PwmUpdate::Stop)
                .expect("Failed to notify SoftPwm worker thread.");
            handle
                .join()
                .expect("Failed to cleanup SoftPwm worker thread.");
        }
    }
}
