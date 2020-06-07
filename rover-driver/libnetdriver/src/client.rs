use either::Either;
use futures::SinkExt;
use futures::StreamExt;
use log::{error, trace};
use tokio::net::TcpStream;
use tokio_serde_cbor::Codec;
use tokio_util::codec::{Decoder, Framed};

use async_trait::async_trait;
use librover::api::{AsyncMover, AsyncLooker, AsyncSensor};

use crate::{Error, Result};
use crate::contract::data;
use crate::contract::data::{MoveType, ProtocolMessage, StatusResponse};

type ChannelType = Framed<TcpStream, Codec<data::ProtocolMessage, data::ProtocolMessage>>;

pub struct Client {
    channel: ChannelType
}

impl Client {
    pub async fn new(driver_address: &str) -> Result<Client> {
        Ok(Client {
            channel: Self::connect(driver_address).await?
        })
    }

    pub async fn reconnect(&mut self, driver_address: &str) -> Result<()> {
        self.channel = Self::connect(driver_address).await?;

        Ok(())
    }

    async fn connect(driver_address: &str) -> Result<ChannelType> {
        let stream = TcpStream::connect(driver_address).await?;

        trace!("[{}] Connected.", driver_address);

        let codec: Codec<data::ProtocolMessage, data::ProtocolMessage> = Codec::new();

        Ok(codec.framed(stream))
    }

    async fn exchange<T, F>(&mut self, request: ProtocolMessage, response_processor: F) -> Result<T>
        where F: Fn(ProtocolMessage) -> Either<Result<T>, ProtocolMessage> {
        trace!("Request to netdriver: {:#?}", request);

        self.channel.send(request).await?;

        match self.channel.next().await {
            Some(response) => match response {
                Ok(message) => {
                    trace!("Response from netdriver: {:#?}", message);

                    match response_processor(message) {
                        Either::Left(value) => value,
                        Either::Right(msg) => Err(Error::Protocol(msg))
                    }
                }
                Err(e) => {
                    error!("Failed to receive message: {}", e);
                    Err(e.into())
                }
            },
            None => {
                error!("Connection closed.");
                Err(Error::Disconnected)
            }
        }
    }

    fn process_status(message: ProtocolMessage) -> Either<Result<()>, ProtocolMessage> {
        if let ProtocolMessage::StatusResponse(status) = message {
            Either::Left(match status {
                StatusResponse::Success => Ok(()),
                StatusResponse::Error(e) => Err(Error::Server(e))
            })
        } else {
            Either::Right(message)
        }
    }
}

#[async_trait]
impl AsyncMover for Client {
    type Error = Error;

    async fn stop(&mut self) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::Forward,
            speed: 0,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn move_forward(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::Forward,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn move_backward(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::Backward,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn spin_right(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::SpinCW,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn spin_left(&mut self, speed: u8) -> Result<()> {
        let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
            move_type: MoveType::SpinCCW,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }
}