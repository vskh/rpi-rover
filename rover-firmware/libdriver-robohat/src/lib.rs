use anyhow::Error as GenError;
use libutil::softpwm::Error as PWMError;
use rppal::gpio::Error as GPIOError;
use std::io::Error as IOError;
use std::time::SystemTimeError;
use thiserror::Error as LibError;

pub use robohat::RobohatRover;

mod robohat;

#[derive(Debug, LibError)]
pub enum Error {
    #[error("Library is incompatible with board hardware: {0:?}")]
    Incompatible(GPIOError),

    #[error("System error: {0:?}")]
    System(GenError),

    #[error("Input/output error: {0:?}")]
    IO(#[from] IOError),

    #[error("Software PWM error: {0:?}")]
    PWM(#[from] PWMError),

    #[error("GPIO usage error: {0:?}")]
    GPIO(GPIOError),
}

impl From<GPIOError> for Error {
    fn from(error: GPIOError) -> Self {
        match error {
            GPIOError::Io(e) => Error::IO(e),
            GPIOError::PinNotAvailable(_) | GPIOError::UnknownModel => {
                Error::Incompatible(error)
            }
            GPIOError::PinUsed(_) => Error::GPIO(error),
            GPIOError::PermissionDenied(_) | GPIOError::ThreadPanic => {
                Error::System(GenError::from(error))
            }
        }
    }
}

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Error::System(GenError::from(error))
    }
}

type Result<T> = std::result::Result<T, Error>;
