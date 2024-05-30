use clap::{arg, command, value_parser, ArgAction, ArgGroup};

use libapi_net::client::Client;
use libdriver::util::a_sync::AsyncRover;
use libdriver_robohat::RobohatRover;
use libux_console::controller::RideController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = command!()
        .arg(
            arg!(local: -l --local "Enable local mode (if running directly on the rover)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(remote: -r --remote <ADDR> "Enable remote mode (if connecting through net API)")
                .value_parser(value_parser!(String)),
        )
        .group(
            ArgGroup::new("mode")
                .args(["local", "remote"])
                .required(true),
        )
        .get_matches();

    if opts.contains_id("local") {
        let async_rover: AsyncRover<RobohatRover> = RobohatRover::new()?.into();
        RideController::new(async_rover)?.run().await?
    } else {
        let rover_address = opts.get_one::<String>("address").unwrap();

        RideController::new(Client::new(rover_address).await?)?
            .run()
            .await?
    }

    Ok(())
}
