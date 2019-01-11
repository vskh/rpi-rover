use std::sync::{Arc, Mutex};
use std::fmt;
use std::fmt::{Display, Formatter};

use rppal::gpio::{Gpio, Mode, Level, Error as RppalError};
use util::SoftPwm;
use rover::api;

// motors control pins in BCM numbering
const GPIO_MOTOR_L1: u8 = 16;
const GPIO_MOTOR_L2: u8 = 19;
const GPIO_MOTOR_R1: u8 = 13;
const GPIO_MOTOR_R2: u8 = 12;

type Result<T> = std::result::Result<T, RobohatError>;

#[derive(Debug)]
pub enum RobohatError {
    Gpio,
    GpioInitiaization(RppalError),
    InvalidGpioChannel(u8),
    InvalidValue(String)
}

impl Display for RobohatError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RobohatError::Gpio =>
                write!(f, "GPIO operation error."),
            RobohatError::GpioInitiaization(inner) =>
                write!(f, "GPIO initialization failed: {}", inner),
            RobohatError::InvalidGpioChannel(pin) =>
                write!(f, "Invalid channel: {}", pin),
            RobohatError::InvalidValue(s) =>
                write!(f, "Invalid value of argument '{}'.", s)
        }
    }
}

pub struct RobohatRover {
    left_motor: (SoftPwm, SoftPwm),
    right_motor: (SoftPwm, SoftPwm),
}

impl RobohatRover {
    pub fn new() -> Result<RobohatRover> {
        let gpio = Arc::new(
            Mutex::new(
                Gpio::new().map_err(
                    |e| -> RobohatError { RobohatError::GpioInitiaization(e) }
                )?
            )
        );

        let lm = (
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_L1, 10.0, 0.0),
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_L2, 10.0, 0.0)
        );

        let rm = (
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_R1, 10.0, 0.0),
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_R2, 10.0, 0.0)
        );

        Ok(
            RobohatRover {
                left_motor: lm,
                right_motor: rm,
            }
        )
    }

    fn set_motor_speed(motor: &mut (SoftPwm, SoftPwm), speed: u8, forward: bool) -> Result<()> {
        let frequency = speed as f32;
        let duty_cycle = speed as f32 / 255.0;

        if speed == 0 {
            motor.0.set_duty_cycle(0.0).map_err(|e| { RobohatError::Gpio });
            motor.1.set_duty_cycle(0.0).map_err(|e| { RobohatError::Gpio });
        } else if forward {
            motor.0.set_duty_cycle(speed as f32 / 100.0).map_err(|e| { RobohatError::Gpio });
            motor.0.set_frequency(speed as f32).map_err(|e| { RobohatError::Gpio });
            motor.1.set_duty_cycle(0.0).map_err(|e| { RobohatError::Gpio });
        } else {
            motor.0.set_duty_cycle(0.0).map_err(|e| { RobohatError::Gpio });
            motor.1.set_duty_cycle(speed as f32 / 100.0).map_err(|e| { RobohatError::Gpio });
            motor.1.set_frequency(speed as f32).map_err(|e| { RobohatError::Gpio });
        }

        Ok(())
    }
}

impl api::Rover for RobohatRover {

    fn stop(&mut self) {
        RobohatRover::set_motor_speed(&mut (self.left_motor), 0, false).unwrap();
        RobohatRover::set_motor_speed(&mut (self.right_motor), 0, false).unwrap();
    }

    fn move_forward(&mut self, speed: u8) {
        RobohatRover::set_motor_speed(&mut (self.left_motor), speed, true).unwrap();
        RobohatRover::set_motor_speed(&mut (self.right_motor), speed, true).unwrap();
    }

    fn move_backward(&mut self, speed: u8) {
        RobohatRover::set_motor_speed(&mut (self.left_motor), speed, false).unwrap();
        RobohatRover::set_motor_speed(&mut (self.right_motor), speed, false).unwrap();
    }

    fn spin_right(&mut self, speed: u8) {
        RobohatRover::set_motor_speed(&mut (self.left_motor), speed, true).unwrap();
        RobohatRover::set_motor_speed(&mut (self.right_motor), speed, false).unwrap();
    }

    fn spin_left(&mut self, speed: u8) {
        RobohatRover::set_motor_speed(&mut (self.left_motor), speed, false).unwrap();
        RobohatRover::set_motor_speed(&mut (self.right_motor), speed, true).unwrap();
    }
}