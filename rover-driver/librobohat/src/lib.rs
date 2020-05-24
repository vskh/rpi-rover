mod robohat;

pub use robohat::RobohatRover;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Pwm,
    Gpio(rppal::gpio::Error),
    Time(std::time::SystemTimeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Pwm => write!(f, "PWM update error."),
            Error::Gpio(inner) => write!(f, "GPIO error: {}", inner),
            Error::Time(inner) => write!(f, "System time error: {}", inner)
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::Pwm => None,
            Error::Gpio(e) => Some(e),
            Error::Time(e) => Some(e)
        }
    }
}

impl From<rppal::gpio::Error> for Error {
    fn from(e: rppal::gpio::Error) -> Self {
        Error::Gpio(e)
    }
}

impl From<libutil::softpwm::Error> for Error {
    fn from(_: libutil::softpwm::Error) -> Self {
        Error::Pwm
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Self {
        Error::Time(e)
    }
}

impl From<Error> for librover::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Gpio(_) => librover::Error::Hardware(Box::new(e)),
            Error::Pwm => librover::Error::Software(Box::new(e)),
            Error::Time(_) => librover::Error::Software(Box::new(e))
        }
    }
}