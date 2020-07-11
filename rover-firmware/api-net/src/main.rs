use log::info;

use libapi_net::server::Server;
use libdriver::util::a_sync::AsyncRover;
use libdriver_robohat::RobohatRover;
use libutil::sys::normalize_path;

mod logger;

const CONFIG_FILE: &str = "Config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover api-net is starting up.");

    let self_path = std::env::current_exe().expect("Could not get current executable path.");
    let _parent_dir = self_path
        .parent()
        .expect("Could not get current executable containing directory.");
    let current_dir = std::env::current_dir().expect("Could not get current working directory.");

    // load settings
    let config_path = normalize_path(CONFIG_FILE, &current_dir);

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(&config_path))?;

    // initialize logging
    logger::init_log(
        settings
            .get_str("log_config")
            .map(|r| normalize_path(&r, &current_dir))
            .ok(),
    )?;

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
