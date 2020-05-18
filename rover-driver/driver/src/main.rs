mod logger;

use log::info;
use libdriver::server::Server;

const CONFIG_FILE: &str = "./Config";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover driver is starting up.");

    // load settings
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(CONFIG_FILE))?;

    // initialize logging
    logger::init_log(settings.get_str("log_config").ok())?;

    let listen_addr = settings.get_str("listen")?;

    info!("Starting driver on {}...", listen_addr);

    // create server
    let mut server = Server::new(&listen_addr)
        .await?;

    // start run loop
    server.serve()
        .await?;

    info!("Rover driver finished.");

    Ok(())
}
