use log::{debug, error, info, trace, warn};
use tokio::net::{TcpListener, TcpStream};

pub async fn dispatch(mut listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Starting dispatch loop.");

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(socket: TcpStream) {
    debug!(
        "Received connection from {}.",
        socket
            .peer_addr()
            .map_or_else(|_| "unknown address".to_owned(), |addr| addr.to_string())
    );
}
