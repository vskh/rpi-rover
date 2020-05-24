use futures::StreamExt;
use futures::SinkExt;
use async_trait::async_trait;
use log::{error, trace};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, Decoder};
use tokio_serde_cbor::Codec;
use crate::{Result, Error};
use crate::contract::data;
use crate::contract::Mover;
use crate::contract::data::{StatusResponse, MoveType, ProtocolMessage};
use either::Either;

type ChannelType = Framed<TcpStream, Codec<data::ProtocolMessage, data::ProtocolMessage>>;

pub struct Client {
    channel: ChannelType
}

impl Client {
    pub async fn new(driver_address: &str) -> Result<Client> {
        Ok(Client {
            channel: Client::connect(driver_address).await?
        })
    }

    pub async fn reconnect(&mut self, driver_address: &str) -> Result<()> {
        self.channel = Client::connect(driver_address).await?;

        Ok(())
    }

    async fn connect(driver_address: &str) -> Result<ChannelType> {
        let stream = TcpStream::connect(driver_address).await?;

        trace!("[{}] Connected.", driver_address);

        let codec: Codec<data::ProtocolMessage, data::ProtocolMessage> = Codec::new();

        Ok(codec.framed(stream))
    }

    async fn await_response<T, F>(&mut self, message_extractor: F) -> Result<T>
        where F: Fn(ProtocolMessage) -> Either<T, ProtocolMessage> {
        match self.channel.next().await {
            Some(response) => match response {
                Ok(message) => {
                    match message_extractor(message) {
                        Either::Left(value) => Ok(value),
                        Either::Right(msg) => Err(Error::Internal(format!("Unexpected response: {:#?}", msg)))
                    }
                }
                Err(e) => {
                    error!("Failed to receive message: {}", e);
                    Err(Error::from(e))
                }
            },
            None => {
                error!("Connection closed.");
                Err(Error::Internal("Connection to the driver lost.".to_owned()))
            }
        }
    }

    fn extract_status(message: ProtocolMessage) -> Either<StatusResponse, ProtocolMessage> {
        if let ProtocolMessage::StatusResponse(status) = message {
            Either::Left(status)
        } else {
            Either::Right(message)
        }
    }
}

#[async_trait]
impl Mover for Client {
    async fn stop(&mut self) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::Forward,
            speed: 0,
        });

        trace!("Request to driver: {:#?}", msg);

        self.channel.send(msg).await?;

        let status = self.await_response(Client::extract_status).await?;

        trace!("Driver response: {:#?}", status);
        
        match status {
            StatusResponse::Success => Ok(()),
            StatusResponse::Error(description) => Err(Error::Internal(description))
        }
    }

    async fn move_forward(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::Forward,
            speed,
        });

        trace!("Request to driver: {:#?}", msg);

        self.channel.send(msg).await?;

        let status = self.await_response(Client::extract_status).await?;

        trace!("Driver response: {:#?}", status);

        match status {
            StatusResponse::Success => Ok(()),
            StatusResponse::Error(description) => Err(Error::Internal(description))
        }
    }

    async fn move_backward(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::Backward,
            speed,
        });

        trace!("Request to driver: {:#?}", msg);

        self.channel.send(msg).await?;

        let status = self.await_response(Client::extract_status).await?;

        trace!("Driver response: {:#?}", status);

        match status {
            StatusResponse::Success => Ok(()),
            StatusResponse::Error(description) => Err(Error::Internal(description))
        }
    }

    async fn spin_right(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::SpinCW,
            speed,
        });

        trace!("Request to driver: {:#?}", msg);

        self.channel.send(msg).await?;

        let status = self.await_response(Client::extract_status).await?;

        trace!("Driver response: {:#?}", status);

        match status {
            StatusResponse::Success => Ok(()),
            StatusResponse::Error(description) => Err(Error::Internal(description))
        }
    }

    async fn spin_left(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::SpinCCW,
            speed,
        });

        trace!("Request to driver: {:#?}", msg);

        self.channel.send(msg).await?;

        let status = self.await_response(Client::extract_status).await?;

        trace!("Driver response: {:#?}", status);

        match status {
            StatusResponse::Success => Ok(()),
            StatusResponse::Error(description) => Err(Error::Internal(description))
        }
    }
}