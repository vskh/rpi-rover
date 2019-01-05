use std::fmt;
use std::fmt::{Display, Formatter};
use rppal::gpio::{Gpio, Mode, Level, Error as RppalError};
//use rppal::pwm::{Pwm, Channel};
use rover::api;

// RaspberryPi model B+ BCM to physical pins map
const GPIO_TO_PIN_REV3: [i8; 33] = [
    -1, -1,  3,  5,  7, 29, 31, 26, 24, 21, 19,
    23, 32, 33,  8, 10, 36, 11, 12, 35, 38, 40,
    15, 16, 18, 22, 37, 13, -1, -1, -1, -1,  0
];

// motors control pins
const GPIO_MOTOR_L1: u8 = 36;
const GPIO_MOTOR_L2: u8 = 35;
const GPIO_MOTOR_R1: u8 = 33;
const GPIO_MOTOR_R2: u8 = 32;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GpioInitiaizationFailure(RppalError),
    InvalidGpioPin(usize)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::GpioInitiaizationFailure(inner) =>
                write!(f, "GPIO initialization failed: {}", inner),
            Error::InvalidGpioPin(pin) =>
                write!(f, "Invalid GPIO pin: {}", pin)
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

    fn bcm2pin(gpio_id: usize) -> Result<u8> {
        let pin_id = GPIO_TO_PIN_REV3[gpio_id];
        if pin_id > 0 {
            Ok(pin_id as u8)
        } else {
            Err(Error::InvalidGpioPin(gpio_id))
        }
    }
}

impl api::Rover for RobohatRover {
    fn stop(&self) {
        self.gpio.write(GPIO_MOTOR_L1, Level::Low);
        self.gpio.write(GPIO_MOTOR_L1, Level::Low);
        self.gpio.write(GPIO_MOTOR_R1, Level::Low);
        self.gpio.write(GPIO_MOTOR_R2, Level::Low);
    }

    fn move_forward(&self, speed: u32) {
        self.gpio.write(GPIO_MOTOR_L1, Level::High);
        self.gpio.write(GPIO_MOTOR_L1, Level::Low);
        self.gpio.write(GPIO_MOTOR_R1, Level::High);
        self.gpio.write(GPIO_MOTOR_R2, Level::Low);
    }
}