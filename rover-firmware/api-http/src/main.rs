use futures::lock::Mutex;

use actix_rt;
use actix_web::{App, HttpServer, web};
use log::info;

use libapi_net::client::Client;
use libutil::app::bootstrap;

mod app;
mod move_api;
mod look_api;
mod sense_api;

const CONFIG_FILE: &str = "Config.toml";

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover api-http is starting up.");

    let settings = bootstrap(CONFIG_FILE)?;

    let listen_addr = settings.get_str("listen_address")?;

    info!("Starting api-http on {}...", listen_addr);

    let rover_addr = settings.get_str("rover_address")?;

    let state = web::Data::new(
        app::State {
            rover_client: Mutex::new(Client::new(rover_addr).await?)
        }
    );

    let app_factory = move || App::new()
        .app_data(state.clone())
        .service(web::scope("/move").configure(move_api::config))
        .service(web::scope("/look").configure(look_api::config))
        .service(web::scope("/sense").configure(sense_api::config));

    HttpServer::new(app_factory)
        .bind(listen_addr)
        .unwrap()
        .run()
        .await?;

    info!("Stopping api-http.");

    Ok(())
}
