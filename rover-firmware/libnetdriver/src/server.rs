use futures::{SinkExt, StreamExt};
use log::{debug, error, info, trace, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde_cbor::Codec;
use tokio_util::codec::Decoder;

use libdriver::api::{Looker, Mover, Sensor};

use crate::contract::data::{MoveType, ProtocolMessage, StatusResponse, SenseRequest, SenseResponse};
use crate::Result;

pub struct Server<TMover, TLooker, TSensor>
    where TMover: Mover, TLooker: Looker, TSensor: Sensor {
    listener: TcpListener,
    mover: Option<TMover>,
    looker: Option<TLooker>,
    sensor: Option<TSensor>,
}

impl<TMover, TLooker, TSensor> Server<TMover, TLooker, TSensor>
    where TMover: Mover, TLooker: Looker, TSensor: Sensor {
    pub async fn new(listen_address: &str) -> Result<Server<TMover, TLooker, TSensor>> {
        info!("Launching netdriver server on {}.", listen_address);

        let listener = TcpListener::bind(listen_address)
            .await?;

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
            let (socket, _) = self.listener
                .accept()
                .await?;

            trace!("New connection accepted.");

            // single connection only
            self.handle_connection(socket).await?;

            trace!("Connection handling finished. Awaiting next.");
        }
    }

    fn map_result_to_status_response<T, E>(r: std::result::Result<T, E>) -> ProtocolMessage
        where E: std::fmt::Display {
        ProtocolMessage::StatusResponse(
            match r {
                Ok(_) => StatusResponse::Success,
                Err(e) => StatusResponse::Error(e.to_string())
            }
        )
    }

    async fn handle_connection(&mut self, socket: TcpStream) -> Result<()> {
        let peer_address = socket
            .peer_addr()
            .map_or("unknown address".to_owned(), |addr| addr.to_string());

        debug!("[{}] New connection received.", peer_address);

        let codec: Codec<ProtocolMessage, ProtocolMessage> = Codec::new();

        let mut channel = codec.framed(socket);

        while let Some(response) = channel.next().await {
            match response {
                Ok(message) => {
                    match &message {
                        ProtocolMessage::MoveRequest(r) => {
                            trace!("[{}] Processing move request: {:#?}", peer_address, r);

                            if let Some(ref mut mover) = self.mover {
                                let opresult = match r.move_type {
                                    MoveType::Forward => mover.move_forward(r.speed),
                                    MoveType::Backward => mover.move_backward(r.speed),
                                    MoveType::SpinCW => mover.spin_right(r.speed),
                                    MoveType::SpinCCW => mover.spin_left(r.speed)
                                };

                                channel
                                    .send(Self::map_result_to_status_response(opresult))
                                    .await?;
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponse::Error("Unsupported operation.".to_owned())))
                                    .await?;
                            }
                        }
                        ProtocolMessage::LookRequest(r) => {
                            trace!("[{}] Processing look request: {:#?}", peer_address, r);

                            if let Some(ref mut looker) = self.looker {
                                let opresult = looker
                                    .look_at(r.x, r.y);

                                channel
                                    .send(Self::map_result_to_status_response(opresult))
                                    .await?;
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponse::Error("Unsupported operation.".to_owned())))
                                    .await?
                            }
                        }
                        ProtocolMessage::SenseRequest(r) => {
                            trace!("[{}] Processing sense request: {:#?}", peer_address, r);

                            if let Some(ref mut sensor) = self.sensor {
                                match r {
                                    SenseRequest::Distance => {
                                        let opresult = match sensor.scan_distance() {
                                            Ok(distance) => SenseResponse::Distance(distance),
                                            Err(e) => SenseResponse::Error(e.to_string())
                                        };

                                        channel
                                            .send(ProtocolMessage::SenseResponse(opresult))
                                            .await?;
                                    },
                                    SenseRequest::Line => {
                                        let opresult = match sensor.get_lines() {
                                            Ok(line_states) => SenseResponse::Line(line_states),
                                            Err(e) => SenseResponse::Error(e.to_string())
                                        };

                                        channel
                                            .send(ProtocolMessage::SenseResponse(opresult))
                                            .await?;
                                    },
                                    SenseRequest::Obstacle => {
                                        let opresult = match sensor.get_obstacles() {
                                            Ok(obstacle_states) => SenseResponse::Obstacle(obstacle_states),
                                            Err(e) => SenseResponse::Error(e.to_string())
                                        };

                                        channel
                                            .send(ProtocolMessage::SenseResponse(opresult))
                                            .await?;
                                    }
                                }
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::SenseResponse(SenseResponse::Error("Unsupported operation.".to_owned())))
                                    .await?
                            }
                        }

                        _ => warn!("[{}] Received unsupported request type: {:#?}", peer_address, message),
                    }

                    debug!("[{}] Successfully processed message: {:#?}", peer_address, message);
                }
                Err(e) => {
                    error!("[{}] Failed to receive message: {}", peer_address, e);
                    return Err(e.into());
                }
            }
        }

        debug!("[{}] Connection terminated.", peer_address);

        Ok(())
    }
}

