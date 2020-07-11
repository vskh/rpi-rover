use log::info;

use libapi_net::server::Server;
use libdriver::util::a_sync::AsyncRover;
use libdriver_robohat::RobohatRover;

use libutil::app::bootstrap;

const CONFIG_FILE: &str = "Config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover api-net is starting up.");

    let settings = bootstrap(CONFIG_FILE)?;

    let listen_addr = settings.get_str("listen")?;

    info!("Starting api-net on {}...", listen_addr);

    // create server
    let mut server = Server::new(&listen_addr).await?;

    // link api-net server with actual rover control implementation
    let async_rover: AsyncRover<RobohatRover> = RobohatRover::new()?.into();

    server.register_mover(Some(async_rover.clone()));
    server.register_looker(Some(async_rover.clone()));
    server.register_sensor(Some(async_rover.clone()));

    // start run loop
    server.serve().await?;

    info!("Rover api-net finished.");

    Ok(())
}
