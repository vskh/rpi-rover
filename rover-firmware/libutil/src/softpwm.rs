use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use log::{error, trace};
use rppal::gpio::{Level, OutputPin};
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
    pub fn new(mut pin: OutputPin, frequency: f32, duty_cycle: f32) -> SoftPwm {
        pin.set_low();

        let (tx, rx) = mpsc::channel();

        SoftPwm {
            channel: tx,
            worker: Some(thread::spawn(move || {
                let mut worker = SoftPwmWorker::new(pin, frequency, duty_cycle, rx);

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
    pin: OutputPin,
    frequency: f32,
    duty_cycle: f32,
    channel: mpsc::Receiver<PwmUpdate>,
    time_on: Duration,
    time_off: Duration,
}

impl SoftPwmWorker {
    fn new(
        pin: OutputPin,
        init_frequency: f32,
        init_duty_cycle: f32,
        channel: mpsc::Receiver<PwmUpdate>,
    ) -> SoftPwmWorker {
        let (time_on_ns, time_off_ns) = SoftPwmWorker::calc_times(init_frequency, init_duty_cycle);

        SoftPwmWorker {
            pin,
            frequency: init_frequency,
            duty_cycle: init_duty_cycle,
            channel,
            time_on: Duration::from_nanos(time_on_ns),
            time_off: Duration::from_nanos(time_off_ns),
        }
    }

    fn calc_times(frequency: f32, duty_cycle: f32) -> (u64, u64) {
        let period_sec = 1.0 / frequency;
        let time_on_sec = period_sec * duty_cycle;
        let time_off_sec = period_sec - time_on_sec;

        trace!("Updated PWM timing: f = {} Hz, T = {} s, duty = {}, on = {} s, off = {} s",
            frequency, period_sec, duty_cycle, time_on_sec, time_off_sec);

        ((time_on_sec * 1000000000.0) as u64, (time_off_sec * 1000000000.0) as u64)
    }

    fn update_times(&mut self) {
        let (time_on_ns, time_off_ns) = SoftPwmWorker::calc_times(self.frequency, self.duty_cycle);
        self.time_on = Duration::from_nanos(time_on_ns);
        self.time_off = Duration::from_nanos(time_off_ns);
    }

    fn check_updates(&mut self, timeout: Duration) -> Option<(Duration, Duration)> {
        let mut updated = false;
        match self.channel.recv_timeout(timeout) {
            Ok(update) =>
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
            Err(_e) => { /* allotted wait time has lapsed */ }
        }

        if updated {
            self.update_times();
        }

        Some((self.time_on, self.time_off))
    }

    fn drive(&mut self, duration: Duration, level: Level) {
        if !duration.is_zero() {
            self.pin.write(level);
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
