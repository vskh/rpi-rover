use either::Either;
use futures::lock::Mutex;
use futures::{FutureExt, SinkExt, StreamExt};
use log::{error, trace};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_serde_cbor::Codec;
use tokio_util::codec::{Decoder, Framed};

use async_trait::async_trait;
use libdriver::api::{AsyncLooker, AsyncMover, AsyncSensor};

use crate::contract::data::{
    LookRequest, MoveRequest, MoveType, ProtocolMessage, SenseRequest, SenseResponse,
    StatusResponse,
};
use crate::{Error, Result};

type ChannelType = Framed<TcpStream, Codec<ProtocolMessage, ProtocolMessage>>;

pub struct Client {
    channel: Mutex<ChannelType>,
}

impl Client {
    pub async fn new<T: ToSocketAddrs>(net_api_address: T) -> Result<Client> {
        Ok(Client {
            channel: Mutex::new(Self::connect(net_api_address).await?),
        })
    }

    pub async fn reconnect<T: ToSocketAddrs>(&mut self, net_api_address: T) -> Result<()> {
        self.channel = Mutex::new(Self::connect(net_api_address).await?);

        Ok(())
    }

    async fn connect<T: ToSocketAddrs>(net_api_address: T) -> Result<ChannelType> {
        let stream = TcpStream::connect(net_api_address).await?;
        let remote_addr = stream.peer_addr().unwrap();

        trace!("[{}] Connected.", remote_addr);

        let codec: Codec<ProtocolMessage, ProtocolMessage> = Codec::new();

        Ok(codec.framed(stream))
    }

    async fn exchange<T, F>(&self, request: ProtocolMessage, response_processor: F) -> Result<T>
    where
        F: Fn(ProtocolMessage) -> Either<Result<T>, ProtocolMessage>,
    {
        trace!("Request to api-net: {:#?}", request);

        self.channel
            .lock()
            .then(|mut guard| async move { guard.send(request).await })
            .await?;

        let r = self
            .channel
            .lock()
            .then(|mut guard| async move { guard.next().await })
            .await;

        match r {
            Some(response) => match response {
                Ok(message) => {
                    trace!("Response from api-net: {:#?}", message);

                    match response_processor(message) {
                        Either::Left(value) => value,
                        Either::Right(msg) => Err(Error::Protocol(msg)),
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
                StatusResponse::Error(e) => Err(Error::Server(e)),
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
        let msg = ProtocolMessage::MoveRequest(MoveRequest {
            move_type: MoveType::Forward,
            speed: 0,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn move_forward(&mut self, speed: u8) -> Result<()> {
        let msg = ProtocolMessage::MoveRequest(MoveRequest {
            move_type: MoveType::Forward,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn move_backward(&mut self, speed: u8) -> Result<()> {
        let msg = ProtocolMessage::MoveRequest(MoveRequest {
            move_type: MoveType::Backward,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn spin_right(&mut self, speed: u8) -> Result<()> {
        let msg = ProtocolMessage::MoveRequest(MoveRequest {
            move_type: MoveType::SpinCW,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }

    async fn spin_left(&mut self, speed: u8) -> Result<()> {
        let msg = ProtocolMessage::MoveRequest(MoveRequest {
            move_type: MoveType::SpinCCW,
            speed,
        });

        self.exchange(msg, Self::process_status).await
    }
}

#[async_trait]
impl AsyncLooker for Client {
    type Error = Error;

    async fn look_at(&mut self, h: i16, v: i16) -> Result<()> {
        let msg = ProtocolMessage::LookRequest(LookRequest { x: h, y: v });

        self.exchange(msg, Self::process_status).await
    }
}

#[async_trait]
impl AsyncSensor for Client {
    type Error = Error;

    async fn get_obstacles(&self) -> Result<Vec<bool>> {
        let msg = ProtocolMessage::SenseRequest(SenseRequest::Obstacle);

        let process_sense_response = |message| {
            if let ProtocolMessage::SenseResponse(sense) = message {
                match sense {
                    SenseResponse::Obstacle(obstacle_data) => Either::Left(Ok(obstacle_data)),
                    SenseResponse::Error(e) => Either::Left(Err(Error::Server(e))),
                    _ => Either::Right(ProtocolMessage::SenseResponse(sense)),
                }
            } else {
                Either::Right(message)
            }
        };

        self.exchange(msg, process_sense_response).await
    }

    async fn get_lines(&self) -> Result<Vec<bool>> {
        let msg = ProtocolMessage::SenseRequest(SenseRequest::Line);

        let process_sense_response = |message| {
            if let ProtocolMessage::SenseResponse(sense) = message {
                match sense {
                    SenseResponse::Line(line_data) => Either::Left(Ok(line_data)),
                    SenseResponse::Error(e) => Either::Left(Err(Error::Server(e))),
                    _ => Either::Right(ProtocolMessage::SenseResponse(sense)),
                }
            } else {
                Either::Right(message)
            }
        };

        self.exchange(msg, process_sense_response).await
    }

    async fn scan_distance(&mut self) -> Result<f32> {
        let msg = ProtocolMessage::SenseRequest(SenseRequest::Distance);

        let process_sense_response = |message| {
            if let ProtocolMessage::SenseResponse(sense) = message {
                match sense {
                    SenseResponse::Distance(distance) => Either::Left(Ok(distance)),
                    SenseResponse::Error(e) => Either::Left(Err(Error::Server(e))),
                    _ => Either::Right(ProtocolMessage::SenseResponse(sense)),
                }
            } else {
                Either::Right(message)
            }
        };

        self.exchange(msg, process_sense_response).await
    }
}
