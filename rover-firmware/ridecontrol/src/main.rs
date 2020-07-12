use clap::{clap_app, crate_authors, crate_description, crate_version};

use libapi_net::client::Client;
use libdriver::util::a_sync::AsyncRover;
use libdriver_robohat::RobohatRover;
use libridecontrol::controller::RideController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = clap_app!(RoverRideController =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@group mode +required =>
            (@arg local: -l --local "Enable local mode (if running directly on rover)")
            (@arg address: -r --remote +takes_value "Enable remote mode (if connecting through net API)")
        )
    ).get_matches();

    if opts.is_present("local") {
        let async_rover: AsyncRover<RobohatRover> = RobohatRover::new()?.into();
        RideController::new(async_rover)?.run().await?
    } else {
        let rover_address = opts.value_of("address").unwrap();

        RideController::new(Client::new(rover_address).await?)?
            .run()
            .await?
    }

    Ok(())
}
