use std::io;
use futures::executor::block_on;
use futures::sink::SinkExt;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, Decoder};
use tokio_serde_cbor::Codec;
use crate::contract::data;
use crate::contract::Mover;
use crate::contract::data::{StatusResponse, MoveType};

pub struct Client {
    channel: Framed<TcpStream, Codec<data::ProtocolMessage, data::ProtocolMessage>>
}

impl Client {
    pub fn new(driver_address: &str) -> io::Result<Client> {
        let stream = block_on(TcpStream::connect(driver_address))?;
        let codec: Codec<data::ProtocolMessage, data::ProtocolMessage> = Codec::new();

        Ok(Client {
            channel: codec.framed(stream)
        })
    }
}

impl Mover for Client {
    fn stop(&mut self) -> io::Result<StatusResponse> {
        // let msg = data::ProtocolMessage::MoveRequest(data::MoveRequest {
        //     move_type: MoveType::Forward,
        //     speed: 0,
        // });
        //
        // block_on(self.channel.send(msg))?
        unimplemented!()
    }

    fn move_forward(&mut self, speed: u8) -> io::Result<StatusResponse> {
        unimplemented!()
    }

    fn move_backward(&mut self, speed: u8) -> io::Result<StatusResponse> {
        unimplemented!()
    }

    fn spin_right(&mut self, speed: u8) -> io::Result<StatusResponse> {
        unimplemented!()
    }

    fn spin_left(&mut self, speed: u8) -> io::Result<StatusResponse> {
        unimplemented!()
    }
}