use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime};

use rppal::gpio::{Gpio, InputPin, IoPin, Level, Mode};

use libdriver::{api, util};
use libdriver::api::RoverMoveDirection;
use libutil::SoftPwm;

use crate::{Error, Result};

// sensor input pins in BCM numbering
const GPIO_IR_L: u8 = 4;
const GPIO_IR_R: u8 = 17;
const GPIO_LINE_L: u8 = 5;
const GPIO_LINE_R: u8 = 27;

const GPIO_SONAR: u8 = 20;
const SOUND_SPEED: u32 = 343000; // in mm/s

// motors control pins in BCM numbering
const GPIO_MOTOR_L1: u8 = 16;
const GPIO_MOTOR_L2: u8 = 19;
const GPIO_MOTOR_R1: u8 = 13;
const GPIO_MOTOR_R2: u8 = 12;

// pan/tilt servo control pins in BCM numbering
#[allow(dead_code)]
const GPIO_PAN_SERVO: u8 = 25;
#[allow(dead_code)]
const GPIO_TILT_SERVO: u8 = 24;

// pan limits
const PAN_L_CUT_DEGREES: i16 = 90;
const PAN_R_CUT_DEGREES: i16 = -90;
const PAN_C_DEGREES: i16 = 0;
const PAN_L_CUT_PWIDTH: i16 = 220;
const PAN_R_CUT_PWIDTH: i16 = 55;
const PAN_C_PWIDTH: i16 = 138;

// tilt limits
const TILT_U_CUT_DEGREES: i16 = -90;
const TILT_D_CUT_DEGREES: i16 = 80;
const TILT_C_DEGREES: i16 = 0;
const TILT_U_CUT_PWIDTH: i16 = 65;
const TILT_D_CUT_PWIDTH: i16 = 210;
const TILT_C_PWIDTH: i16 = 138;

// Servoblaster control
const SERVOBLASTER: &str = "/extdev/servoblaster";

pub struct RobohatRover {
    sonar_pin: IoPin,
    left_ir_pin: InputPin,
    right_ir_pin: InputPin,
    left_line_pin: InputPin,
    right_line_pin: InputPin,
    left_motor: (SoftPwm, SoftPwm),
    right_motor: (SoftPwm, SoftPwm),
    move_direction: RoverMoveDirection,
    look_direction: (i16, i16)
}

impl RobohatRover {
    pub fn new() -> Result<RobohatRover> {
        let gpio = Gpio::new()?;

        let sonar_pin = gpio.get(GPIO_SONAR)?.into_io(Mode::Output);

        let left_ir_pin = gpio.get(GPIO_IR_L)?.into_input();
        let right_ir_pin = gpio.get(GPIO_IR_R)?.into_input();

        let left_line_pin = gpio.get(GPIO_LINE_L)?.into_input();
        let right_line_pin = gpio.get(GPIO_LINE_R)?.into_input();

        let pin_motor_l1 = gpio.get(GPIO_MOTOR_L1)?.into_output();
        let pin_motor_l2 = gpio.get(GPIO_MOTOR_L2)?.into_output();
        let left_motor = (
            SoftPwm::new(pin_motor_l1, 10.0, 0.0),
            SoftPwm::new(pin_motor_l2, 10.0, 0.0),
        );

        let pin_motor_r1 = gpio.get(GPIO_MOTOR_R1)?.into_output();
        let pin_motor_r2 = gpio.get(GPIO_MOTOR_R2)?.into_output();
        let right_motor = (
            SoftPwm::new(pin_motor_r1, 10.0, 0.0),
            SoftPwm::new(pin_motor_r2, 10.0, 0.0),
        );

        let move_direction_vector = RoverMoveDirection::new((0, 0));
        let look_direction = (0, 0);

        Ok(RobohatRover {
            sonar_pin,
            left_ir_pin,
            right_ir_pin,
            left_line_pin,
            right_line_pin,
            left_motor,
            right_motor,
            move_direction: move_direction_vector,
            look_direction
        })
    }

    fn set_motor_speed(motor: &mut (SoftPwm, SoftPwm), speed: u8, forward: bool) -> Result<()> {
        let frequency = 100_f32;
        let duty_cycle = speed as f32 / 255.0;

        if speed == 0 {
            motor.0.set_duty_cycle(0.0)?;
            motor.1.set_duty_cycle(0.0)?;
        } else if forward {
            motor.0.set_duty_cycle(duty_cycle)?;
            motor.0.set_frequency(frequency)?;
            motor.1.set_duty_cycle(0.0)?;
        } else {
            motor.0.set_duty_cycle(0.0)?;
            motor.1.set_frequency(frequency)?;
            motor.1.set_duty_cycle(duty_cycle)?;
        }

        Ok(())
    }

    fn map_degrees_to_pulse_width(h: i16, v: i16) -> (i16, i16) {
        let deg_to_pw = |deg: i16,
                         e1_deg: i16,
                         e2_deg: i16,
                         mid_deg: i16,
                         e1_pw: i16,
                         e2_pw: i16,
                         mid_pw: i16|
         -> i16 {
            let deg_lo = e1_deg.min(e2_deg);
            let deg_hi = e1_deg.max(e2_deg);
            let deg_span = deg_hi - deg_lo;

            let pw_lo = e1_pw.min(e2_pw);
            let pw_hi = e1_pw.max(e2_pw);
            let pw_span = pw_hi - pw_lo;

            let cvt_coef = pw_span as f32 / deg_span as f32;

            let pw = mid_pw as f32 + ((deg - mid_deg) as f32 * cvt_coef);

            if pw > pw_hi as f32 {
                pw_hi
            } else if pw < pw_lo as f32 {
                pw_lo
            } else {
                pw.round() as i16
            }
        };

        let pan_pw = deg_to_pw(
            h,
            PAN_R_CUT_DEGREES,
            PAN_L_CUT_DEGREES,
            PAN_C_DEGREES,
            PAN_R_CUT_PWIDTH,
            PAN_L_CUT_PWIDTH,
            PAN_C_PWIDTH,
        );
        let tilt_pw = deg_to_pw(
            v,
            TILT_D_CUT_DEGREES,
            TILT_U_CUT_DEGREES,
            TILT_C_DEGREES,
            TILT_D_CUT_PWIDTH,
            TILT_U_CUT_PWIDTH,
            TILT_C_PWIDTH,
        );

        (pan_pw, tilt_pw)
    }
}

impl api::Mover for RobohatRover {
    type Error = Error;

    fn stop(&mut self) -> Result<()> {
        RobohatRover::set_motor_speed(&mut self.left_motor, 0, false)?;
        self.move_direction.update(Some(0), None);

        RobohatRover::set_motor_speed(&mut self.right_motor, 0, false)?;
        self.move_direction.update(None, Some(0));

        Ok(())
    }

    fn move_forward(&mut self, speed: u8) -> Result<()> {
        RobohatRover::set_motor_speed(&mut self.left_motor, speed, true)?;
        self.move_direction.update(Some(speed as i16), None);

        RobohatRover::set_motor_speed(&mut self.right_motor, speed, true)?;
        self.move_direction.update(None, Some(speed as i16));

        Ok(())
    }

    fn move_backward(&mut self, speed: u8) -> Result<()> {
        RobohatRover::set_motor_speed(&mut self.left_motor, speed, false)?;
        self.move_direction.update(Some(- (speed as i16)), None);

        RobohatRover::set_motor_speed(&mut self.right_motor, speed, false)?;
        self.move_direction.update(None, Some(- (speed as i16)));

        Ok(())
    }

    fn spin_right(&mut self, speed: u8) -> Result<()> {
        RobohatRover::set_motor_speed(&mut self.left_motor, speed, true)?;
        self.move_direction.update(Some(speed as i16), None);

        RobohatRover::set_motor_speed(&mut self.right_motor, speed, false)?;
        self.move_direction.update(None, Some(- (speed as i16)));
        Ok(())
    }

    fn spin_left(&mut self, speed: u8) -> Result<()> {
        RobohatRover::set_motor_speed(&mut self.left_motor, speed, false)?;
        self.move_direction.update(Some(- (speed as i16)), None);

        RobohatRover::set_motor_speed(&mut self.right_motor, speed, true)?;
        self.move_direction.update(None, Some(speed as i16));

        Ok(())
    }

    fn get_move_direction(&self) -> std::result::Result<RoverMoveDirection, Self::Error> {
        Ok(self.move_direction)
    }

    fn reset(&mut self) -> Result<()> {
        self.stop()
    }
}

impl api::Looker for RobohatRover {
    type Error = Error;

    fn look_at(&mut self, h: i16, v: i16) -> Result<()> {
        let (hpw, vpw) = RobohatRover::map_degrees_to_pulse_width(h, v);

        //        println!("Converted coordinates: [{}; {}]", hpw, vpw);

        let mut servo_ctl = OpenOptions::new()
            .write(true)
            .open(SERVOBLASTER)
            .expect("Failed to open Servoblaster device.");

        servo_ctl.write(format!("7={}\n", hpw).as_bytes())?;
        servo_ctl.write(format!("6={}\n", vpw).as_bytes())?;

        self.look_direction = (h, v);

        Ok(())
    }

    fn get_look_direction(&self) -> Result<(i16, i16)> {
        Ok(self.look_direction)
    }
}

impl api::Sensor for RobohatRover {
    type Error = Error;

    fn get_obstacles(&self) -> Result<Vec<bool>> {
        Ok(vec![
            self.left_ir_pin.read() == Level::Low,
            self.right_ir_pin.read() == Level::Low,
        ])
    }

    fn get_lines(&self) -> Result<Vec<bool>> {
        Ok(vec![
            self.left_line_pin.read() == Level::Low,
            self.right_line_pin.read() == Level::Low,
        ])
    }

    fn scan_distance(&mut self) -> Result<f32> {
        self.sonar_pin.set_mode(Mode::Output);
        self.sonar_pin.set_high();
        thread::sleep(Duration::from_micros(10));
        self.sonar_pin.set_low();

        self.sonar_pin.set_mode(Mode::Input);

        let timeout = Duration::from_millis(100);
        let mut timeout_guard = SystemTime::now();
        let mut pulse_start = timeout_guard.clone();
        while self.sonar_pin.read() == Level::Low && timeout_guard.elapsed()? < timeout {
            pulse_start = SystemTime::now();
        }

        timeout_guard = SystemTime::now();
        let mut pulse_end = timeout_guard.clone();
        while self.sonar_pin.read() == Level::High && timeout_guard.elapsed()? < timeout {
            pulse_end = SystemTime::now();
        }

        let pulse_width = pulse_end.duration_since(pulse_start)?;

        let pulse_width_f32 =
            pulse_width.as_secs() as f32 + pulse_width.subsec_nanos() as f32 / 1000000000.0;

        let distance = SOUND_SPEED as f32 * pulse_width_f32;

        Ok(distance / 2.0)
    }
}

impl util::splittable::SplittableRover for RobohatRover {}
