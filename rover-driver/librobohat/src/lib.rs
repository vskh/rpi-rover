use thiserror::Error as LibError;

pub use robohat::RobohatRover;

mod robohat;

#[derive(Debug, LibError)]
pub enum Error {
    #[error("PWM update error: {0:?}")]
    Pwm(#[from] libutil::softpwm::Error),

    #[error("GPIO error: {0:?}")]
    Gpio(#[from] rppal::gpio::Error),

    #[error("System time error: {0:?}")]
    Time(#[from] std::time::SystemTimeError),
}

type Result<T> = std::result::Result<T, Error>;
