use log::{debug, error, info, trace, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde_cbor::Codec;
use tokio_util::codec::Decoder;
use futures::stream::StreamExt;
use libdriver::contract::data::ProtocolMessage;

pub async fn dispatch(mut listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Starting dispatch loop.");

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(socket: TcpStream) {
    let peer_address = socket
        .peer_addr()
        .map_or_else(|_| "unknown address".to_owned(), |addr| addr.to_string());

    debug!("Received connection from {}.", peer_address);

    let codec: Codec<ProtocolMessage, ProtocolMessage> = Codec::new();

    let mut channel = codec.framed(socket);

    channel.for_each(|m| {
        match m {
            Ok(protocol_message) => match protocol_message {
                ProtocolMessage::MoveRequest(r) => debug!("Received move request: {:#?}", r),
                ProtocolMessage::LookRequest(r) => debug!("Received look request: {:#?}", r),
                ProtocolMessage::SenseRequest(r) => debug!("Received sense request: {:#?}", r),
                ProtocolMessage::SenseSubscribeRequest(r) => debug!("Received sense subscribe request: {:#?}", r),
                _ => debug!("Received request: {:#?}", protocol_message),
            },
            Err(e) => {
                error!("Message deserialization failed while talking to {}: {}", peer_address, e);
            }
        }

        futures::future::ready(())
    });
}
