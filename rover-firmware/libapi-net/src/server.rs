use futures::{SinkExt, StreamExt};
use log::{debug, error, info, trace, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde_cbor::Codec;
use tokio_util::codec::Decoder;

use libdriver::api::{AsyncLooker, AsyncMover, AsyncSensor};

use crate::{Error, Result};
use crate::contract::data::{
    LookData, MoveType, ProtocolMessage, SenseRequestData, SenseResponseData, StatusResponseData,
};

pub struct Server<TMover, TLooker, TSensor>
where
    TMover: AsyncMover + Send,
    TLooker: AsyncLooker + Send,
    TSensor: AsyncSensor + Send,
{
    listener: TcpListener,
    mover: Option<TMover>,
    looker: Option<TLooker>,
    sensor: Option<TSensor>,
}

impl<TMover, TLooker, TSensor> Server<TMover, TLooker, TSensor>
where
    TMover: AsyncMover + Send,
    TLooker: AsyncLooker + Send,
    TSensor: AsyncSensor + Send,
{
    pub async fn new(listen_address: &str) -> Result<Server<TMover, TLooker, TSensor>> {
        info!("Launching api-net server on {}.", listen_address);

        trace!("Opening TCP listener on {}.", listen_address);
        let listener = TcpListener::bind(listen_address).await?;
        trace!("Listener opened.");

        Ok(Server {
            listener,
            mover: None,
            looker: None,
            sensor: None,
        })
    }

    pub fn register_mover(&mut self, mover: Option<TMover>) {
        self.mover = mover;
    }

    pub fn register_looker(&mut self, looker: Option<TLooker>) {
        self.looker = looker;
    }

    pub fn register_sensor(&mut self, sensor: Option<TSensor>) {
        self.sensor = sensor;
    }

    pub async fn serve(&mut self) -> Result<()> {
        self.dispatch().await
    }

    async fn dispatch(&mut self) -> Result<()> {
        trace!("Starting dispatch loop.");

        loop {
            let (socket, _) = self.listener.accept().await?;

            trace!("New connection accepted.");

            // single connection only
            self.handle_connection(socket).await?;

            trace!("Connection handling finished. Awaiting next.");
        }
    }

    fn map_result_to_status_response<T, E>(r: std::result::Result<T, E>) -> ProtocolMessage
    where
        E: std::fmt::Display,
    {
        ProtocolMessage::StatusResponse(match r {
            Ok(_) => StatusResponseData::Success,
            Err(e) => StatusResponseData::Error(e.to_string()),
        })
    }

    async fn reset(&mut self) -> Result<()> {
        fn to_server_err<T: std::error::Error>(e: T) -> Error {
            Error::Server(e.to_string())
        }

        if let Some(ref mut mover) = self.mover {
            mover.reset().await.map_err(to_server_err)?;
        }

        if let Some(ref mut looker) = self.looker {
            looker.reset().await.map_err(to_server_err)?;
        }

        if let Some(ref mut sensor) = self.sensor {
            sensor.reset().await.map_err(to_server_err)?;
        }

        Ok(())
    }

    async fn handle_connection(&mut self, socket: TcpStream) -> Result<()> {
        let peer_address = socket
            .peer_addr()
            .map_or("unknown address".to_owned(), |addr| addr.to_string());

        debug!("[{}] New connection received.", peer_address);

        let codec: Codec<ProtocolMessage, ProtocolMessage> = Codec::new();
        let mut channel = codec.framed(socket);

        trace!("Resetting rover controls.");

        self.reset().await?;

        while let Some(response) = channel.next().await {
            match response {
                Ok(message) => {
                    match &message {
                        ProtocolMessage::MoveRequest(r) => {
                            trace!("[{}] Processing move request: {:#?}", peer_address, r);

                            if let Some(ref mut mover) = self.mover {
                                let opresult = match r.move_type {
                                    MoveType::Forward => mover.move_forward(r.speed).await,
                                    MoveType::Backward => mover.move_backward(r.speed).await,
                                    MoveType::SpinCW => mover.spin_right(r.speed).await,
                                    MoveType::SpinCCW => mover.spin_left(r.speed).await,
                                };

                                channel
                                    .send(Self::map_result_to_status_response(opresult))
                                    .await?;
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponseData::Error(
                                        "Unsupported operation.".to_owned(),
                                    )))
                                    .await?;
                            }
                        }
                        ProtocolMessage::LookRequest(r) => {
                            trace!("[{}] Processing look request: {:#?}", peer_address, r);

                            if let Some(ref mut looker) = self.looker {
                                let opresult = looker.look_at(r.x, r.y).await;

                                channel
                                    .send(Self::map_result_to_status_response(opresult))
                                    .await?;
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponseData::Error(
                                        "Unsupported operation.".to_owned(),
                                    )))
                                    .await?
                            }
                        }
                        ProtocolMessage::LookDirectionRequest => {
                            trace!(
                                "[{}] Processing look direction request.",
                                peer_address
                            );

                            if let Some(ref mut looker) = self.looker {
                                let response = match looker.get_look_direction().await {
                                    Ok((h, v)) => ProtocolMessage::LookDirectionResponse(LookData { x: h, y: v }),
                                    Err(e) => ProtocolMessage::StatusResponse(StatusResponseData::Error(e.to_string()))
                                };

                                channel
                                    .send(response)
                                    .await?;
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponseData::Error(
                                        "Unsupported operation.".to_owned(),
                                    )))
                                    .await?
                            }
                        }
                        ProtocolMessage::SenseRequest(r) => {
                            trace!("[{}] Processing sense request: {:#?}", peer_address, r);

                            if let Some(ref mut sensor) = self.sensor {
                                match r {
                                    SenseRequestData::Distance => {
                                        let response = match sensor.scan_distance().await {
                                            Ok(distance) => ProtocolMessage::SenseResponse(SenseResponseData::Distance(distance)),
                                            Err(e) => ProtocolMessage::StatusResponse(StatusResponseData::Error(e.to_string())),
                                        };

                                        channel
                                            .send(response)
                                            .await?;
                                    }
                                    SenseRequestData::Line => {
                                        let response = match sensor.get_lines().await {
                                            Ok(line_states) => ProtocolMessage::SenseResponse(SenseResponseData::Line(line_states)),
                                            Err(e) => ProtocolMessage::StatusResponse(StatusResponseData::Error(e.to_string())),
                                        };

                                        channel
                                            .send(response)
                                            .await?;
                                    }
                                    SenseRequestData::Obstacle => {
                                        let response = match sensor.get_obstacles().await {
                                            Ok(obstacle_states) =>
                                                ProtocolMessage::SenseResponse(SenseResponseData::Obstacle(obstacle_states)),
                                            Err(e) => ProtocolMessage::StatusResponse(StatusResponseData::Error(e.to_string())),
                                        };

                                        channel
                                            .send(response)
                                            .await?;
                                    }
                                }
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponseData::Error(
                                        "Unsupported operation.".to_owned(),
                                    )))
                                    .await?
                            }
                        }

                        _ => warn!(
                            "[{}] Received unsupported request type: {:#?}",
                            peer_address, message
                        ),
                    }

                    debug!(
                        "[{}] Successfully processed message: {:#?}",
                        peer_address, message
                    );
                }
                Err(e) => {
                    error!("[{}] Failed to receive message: {}", peer_address, e);
                    return Err(e.into());
                }
            }
        }

        self.reset().await?;

        debug!("[{}] Connection terminated.", peer_address);

        Ok(())
    }
}
