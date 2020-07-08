use log::info;

use libdriver_robohat::RobohatRover;
use libapi_net::server::Server;
use libdriver::util::splittable::SplittableRover;

mod logger;

const CONFIG_FILE: &str = "./Config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover api-net is starting up.");

    // load settings
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(CONFIG_FILE))?;

    // initialize logging
    logger::init_log(settings.get_str("log_config").ok())?;

    let listen_addr = settings.get_str("listen")?;

    info!("Starting api-net on {}...", listen_addr);

    // create server
    let mut server = Server::new(&listen_addr)
        .await?;

    // link api-net server with actual rover control implementation
    let mut rover = RobohatRover::new()?;

    let (mover, looker, sensor) = rover.split();

    server.register_mover(Some(mover));
    server.register_looker(Some(looker));
    server.register_sensor(Some(sensor));

    // start run loop
    server.serve().await?;

    info!("Rover api-net finished.");

    Ok(())
}
