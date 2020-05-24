pub mod api;
pub mod util;

#[derive(Debug)]
pub enum Error {
    Hardware(Box<dyn std::error::Error>),
    Software(Box<dyn std::error::Error>)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Hardware(e) => write!(f, "Hardware error: {}", e),
            Error::Software(e) => write!(f, "Software error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::Hardware(e) => Some(&**e),
            Error::Software(e) => Some(&**e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Software(Box::new(e))
    }
}
