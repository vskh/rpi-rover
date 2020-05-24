use futures::{SinkExt, StreamExt};
use log::{debug, error, warn, info, trace};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde_cbor::Codec;
use tokio_util::codec::Decoder;

use librover::api::{Looker, Mover, Sensor};

use crate::{Result, Error};
use crate::contract::data::{MoveType, ProtocolMessage, StatusResponse};

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
        info!("Launching driver server on {}.", listen_address);

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

    async fn handle_connection(&mut self, socket: TcpStream) -> Result<()> {
        let peer_address = socket
            .peer_addr()
            .map_or_else(|_| "unknown address".to_owned(), |addr| addr.to_string());

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
                                match r.move_type {
                                    MoveType::Forward => mover.move_forward(r.speed)?,
                                    MoveType::Backward => mover.move_backward(r.speed)?,
                                    MoveType::SpinCW => mover.spin_right(r.speed)?,
                                    MoveType::SpinCCW => mover.spin_left(r.speed)?
                                }

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponse::Success))
                                    .await?
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponse::Error("Unsupported operation.".to_owned())))
                                    .await?
                            }
                        }
                        ProtocolMessage::LookRequest(r) => {
                            trace!("[{}] Received look request: {:#?}", peer_address, r);

                            if let Some(ref mut looker) = self.looker {
                                looker.look_at(r.x, r.y)?
                            } else {
                                warn!("[{}] Requested operation is not implemented.", peer_address);

                                channel
                                    .send(ProtocolMessage::StatusResponse(StatusResponse::Error("Unsupported operation.".to_owned())))
                                    .await?
                            }
                        }
                        ProtocolMessage::SenseRequest(r) => {
                            trace!("[{}] Received sense request: {:#?}", peer_address, r)
                        }
                        ProtocolMessage::SenseSubscribeRequest(r) => {
                            trace!("[{}] Received sense subscribe request: {:#?}", peer_address, r)
                        }
                        _ => warn!("[{}] Received unsupported request type: {:#?}", peer_address, message),
                    }

                    debug!("[{}] Successfully processed message: {:#?}", peer_address, message);
                }
                Err(e) => {
                    error!("[{}] Failed to receive message: {}", peer_address, e);
                    return Err(Error::from(e));
                }
            }
        }

        debug!("[{}] Connection terminated.", peer_address);

        Ok(())
    }
}

