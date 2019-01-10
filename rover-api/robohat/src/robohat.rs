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

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GpioInitiaizationFailure(RppalError),
    InvalidGpioChannel(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::GpioInitiaizationFailure(inner) =>
                write!(f, "GPIO initialization failed: {}", inner),
            Error::InvalidGpioChannel(pin) =>
                write!(f, "Invalid channel: {}", pin)
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
                    |e| -> Error { Error::GpioInitiaizationFailure(e) }
                )?
            )
        );

        let lm = (
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_L1, 20.0, 0.0),
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_L2, 20.0, 0.0)
        );

        let rm = (
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_R1, 20.0, 0.0),
            SoftPwm::new(Arc::clone(&gpio), GPIO_MOTOR_R2, 20.0, 0.0)
        );

        Ok(RobohatRover {
            left_motor: lm,
            right_motor: rm
        })
    }
}

impl api::Rover for RobohatRover {
    fn stop(&mut self) {
        self.left_motor.0.set_duty_cycle(0.0);
        self.left_motor.1.set_duty_cycle(0.0);
        self.right_motor.0.set_duty_cycle(0.0);
        self.right_motor.1.set_duty_cycle(0.0);
    }

    fn move_forward(&mut self, speed: f32) {
        self.left_motor.0.set_duty_cycle(speed);
        self.left_motor.1.set_duty_cycle(0.0);
        self.right_motor.0.set_duty_cycle(speed);
        self.right_motor.1.set_duty_cycle(0.0);
    }

    fn move_backward(&mut self, speed: f32) {
        self.left_motor.0.set_duty_cycle(0.0);
        self.left_motor.1.set_duty_cycle(speed);
        self.right_motor.0.set_duty_cycle(0.0);
        self.right_motor.1.set_duty_cycle(speed);
    }

    fn spin_left(&mut self, speed: f32) {
        self.left_motor.0.set_duty_cycle(0.0);
        self.left_motor.1.set_duty_cycle(speed);
        self.right_motor.0.set_duty_cycle(speed);
        self.right_motor.1.set_duty_cycle(0.0);
    }

    fn spin_right(&mut self, speed: f32) {
        self.left_motor.0.set_duty_cycle(speed);
        self.left_motor.1.set_duty_cycle(0.0);
        self.right_motor.0.set_duty_cycle(0.0);
        self.right_motor.1.set_duty_cycle(speed);
    }
}