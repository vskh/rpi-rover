use actix_rt;
use actix_web::{App, HttpServer, web};
use log::info;

use libutil::app::bootstrap;

mod move_api;
mod look_api;
mod sense_api;

const CONFIG_FILE: &str = "Config.toml";

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover api-http is starting up.");

    let settings = bootstrap(CONFIG_FILE)?;

    let listen_addr = settings.get_str("listen")?;

    info!("Starting api-http on {}...", listen_addr);

    let app_factory = || App::new()
        .service(
            web::scope("/api")
                .service(web::scope("/move").configure(move_api::config))
                .service(web::scope("/look").configure(look_api::config))
                .service(web::scope("/sense").configure(sense_api::config))
        );

    HttpServer::new(app_factory)
        .bind(listen_addr)
        .unwrap()
        .run()
        .await?;

    info!("Stopping api-http.");

    Ok(())
}
