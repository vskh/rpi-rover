use thiserror::Error as LibError;

use crate::contract::data::ProtocolMessage;

pub mod contract;
pub mod client;
pub mod server;

#[derive(Debug, LibError)]
pub enum Error {
    #[error("Client error: {0:?}")]
    Client(String),

    #[error("No connection to api-net server.")]
    Disconnected,

    #[error("IO error: {0:?}")]
    IO(#[from] std::io::Error),

    #[error("Protocol error: {0:?}")]
    Protocol(ProtocolMessage),

    #[error("Serialization error: {0:?}")]
    Serialization(#[from] tokio_serde_cbor::Error),

    #[error("Server error: {0:?}")]
    Server(String),
}

pub type Result<T> = std::result::Result<T, Error>;