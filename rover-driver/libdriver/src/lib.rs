pub mod contract;
pub mod client;
pub mod server;

#[derive(Debug)]
pub enum Error {
    Driver(String),
    Io(std::io::Error),
    Serialization(Box<dyn std::error::Error>),
    Unknown(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Driver(description) => write!(f, "{}", description),
            Error::Io(e) => write!(f, "IO Error: {}", e),
            Error::Serialization(e) => write!(f, "Serialization error: {}", e),
            Error::Unknown(e) => write!(f, "Unknown error: {}", e)
        }
    }
}

impl From<tokio_serde_cbor::Error> for Error {
    fn from(e: tokio_serde_cbor::Error) -> Self {
        match e {
            tokio_serde_cbor::Error::Io(io_err) => Error::Io(io_err),
            tokio_serde_cbor::Error::Cbor(cbor_err) => Error::Serialization(Box::new(cbor_err)),
            err => Error::Unknown(Box::new(err))
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::Driver(_) => None,
            Error::Io(e) => Some(e),
            Error::Serialization(e) => Some(&**e),
            Error::Unknown(e) => Some(&**e)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;