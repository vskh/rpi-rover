mod logger;
mod protocol;

use log::{trace, debug, info, warn, error};
use tokio::net::TcpListener;

const CONFIG_FILE: &str = "./Config";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load settings
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(CONFIG_FILE))?;

    // initialize logging
    logger::init_log(settings.get_str("log_config").ok())?;

    let listen_addr = settings.get_str("listen")?;

    info!("Starting driver on {}...", listen_addr);

    // bind socket
    let listener = TcpListener::bind(listen_addr).await?;

    // start dispatch loop
    protocol::dispatch(listener).await
}
