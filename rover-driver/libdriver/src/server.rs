use futures::{SinkExt, StreamExt};
use log::{debug, error, info, trace};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde_cbor::Codec;
use tokio_util::codec::Decoder;

use librover::api::{Looker, Mover, Sensor};

use crate::{Result, Error};
use crate::contract::data::{MoveType, ProtocolMessage, StatusResponse};

pub struct Server {
    listener: TcpListener,
    mover: Option<Box<dyn Mover>>,
    looker: Option<Box<dyn Looker>>,
    sensor: Option<Box<dyn Sensor>>,
}

impl Server {
    pub async fn new(listen_address: &str) -> Result<Server> {
        let listener = TcpListener::bind(listen_address)
            .await?;

        Ok(Server {
            listener,
            mover: None,
            looker: None,
            sensor: None,
        })
    }

    pub fn register_mover(&mut self, mover: Option<Box<dyn Mover>>) {
        self.mover = mover;
    }

    pub fn register_looker(&mut self, looker: Option<Box<dyn Looker>>) {
        self.looker = looker;
    }

    pub fn register_sensor(&mut self, sensor: Option<Box<dyn Sensor>>) {
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

        while let Some(message) = channel.next().await {
            match message {
                Ok(protocol_message) => {
                    match &protocol_message {
                        ProtocolMessage::MoveRequest(r) => {
                            debug!("[{}] Processing move request: {:#?}", peer_address, r);

                            if let Some(ref mut mover) = self.mover {
                                match r.move_type {
                                    MoveType::Forward => mover.move_forward(r.speed),
                                    MoveType::Backward => mover.move_backward(r.speed),
                                    MoveType::SpinCW => mover.spin_right(r.speed),
                                    MoveType::SpinCCW => mover.spin_left(r.speed)
                                }
                            }

                            channel
                                .send(ProtocolMessage::StatusResponse(StatusResponse::Success))
                                .await?
                        }
                        ProtocolMessage::LookRequest(r) => {
                            debug!("[{}] Received look request: {:#?}", peer_address, r)
                        }
                        ProtocolMessage::SenseRequest(r) => {
                            debug!("[{}] Received sense request: {:#?}", peer_address, r)
                        }
                        ProtocolMessage::SenseSubscribeRequest(r) => {
                            debug!("[{}] Received sense subscribe request: {:#?}", peer_address, r)
                        }
                        _ => debug!("[{}] Received unsupported request: {:#?}", peer_address, protocol_message),
                    }
                    info!("[{}] Successfully processed message: {:#?}", peer_address, protocol_message);
                }
                Err(e) => {
                    error!("Failed to receive message while talking to {}.", peer_address);
                    return Err(Error::from(e));
                }
            }
        }

        debug!("[{}] Connection terminated.", peer_address);

        Ok(())
    }
}

