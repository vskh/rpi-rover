use std::fmt;
use std::fmt::{Display, Formatter};

use rppal::gpio::{Gpio, Mode, Level, Error as RppalError};
//use rppal::pwm::{Pwm, Channel};
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
    InvalidGpioChannel(u8)
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
    gpio: Gpio
}

impl RobohatRover {
    pub fn new() -> Result<RobohatRover> {
        Ok(RobohatRover {
            gpio: RobohatRover::init(
                Gpio::new().map_err(
                    |e| -> Error { Error::GpioInitiaizationFailure(e) }
                )?
            )?
        })
    }

    fn init(mut gpio: Gpio) -> Result<Gpio> {
        gpio.set_mode(GPIO_MOTOR_L1, Mode::Output);
        gpio.write(GPIO_MOTOR_L1, Level::Low);
        gpio.set_mode(GPIO_MOTOR_L2, Mode::Output);
        gpio.write(GPIO_MOTOR_L2, Level::Low);
        gpio.set_mode(GPIO_MOTOR_R1, Mode::Output);
        gpio.write(GPIO_MOTOR_R1, Level::Low);
        gpio.set_mode(GPIO_MOTOR_R2, Mode::Output);
        gpio.write(GPIO_MOTOR_R2, Level::Low);

        Ok(gpio)
    }
}

impl api::Rover for RobohatRover {
    fn stop(&self) {
        self.gpio.write(GPIO_MOTOR_L1, Level::Low);
        self.gpio.write(GPIO_MOTOR_L2, Level::Low);
        self.gpio.write(GPIO_MOTOR_R1, Level::Low);
        self.gpio.write(GPIO_MOTOR_R2, Level::Low);
    }

    fn move_forward(&self, speed: u32) {
        self.gpio.write(GPIO_MOTOR_L1, Level::High);
        self.gpio.write(GPIO_MOTOR_L2, Level::Low);
        self.gpio.write(GPIO_MOTOR_R1, Level::High);
        self.gpio.write(GPIO_MOTOR_R2, Level::Low);
    }
}