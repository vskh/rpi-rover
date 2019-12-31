mod logger;

use log::{trace, debug, info, warn, error};
use tokio::net::TcpListener;
use tokio::prelude::*;

const CONFIG_FILE: &str = "./Config";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load settings
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(CONFIG_FILE))?;

    // initialize logging
    logger::init_log(settings.get_str("log_config").ok())?;

    // bind socket
    let mut listener = TcpListener::bind(settings.get_str("listen")?).await?;

    // start accepting connections
    loop {
        let (mut socket, client_addr) = listener.accept().await?;

        tokio::spawn(async move {
            debug!("Received connection from {}.", client_addr);
        });
    }

    Ok(())
}
