use actix_web::{App, server};
use log::info;

use libutil::app::bootstrap;

mod controllers;

const CONFIG_FILE: &str = "Config.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Rover api-http is starting up.");

    let settings = bootstrap(CONFIG_FILE)?;

    let listen_addr = settings.get_str("listen")?;

    info!("Starting api-http on {}...", listen_addr);

    server::new(|| {
        App::new()
            .resource("/move/forward", |r| r.f(controllers::move_forward))
            .resource("/move/backward", |r| r.f(controllers::move_backward))
            .resource("/spin/left", |r| r.f(controllers::spin_left))
            .resource("/spin/right", |r| r.f(controllers::spin_right))
            .resource("/look", |r| r.f(controllers::look))
            .resource("/get/obstacles", |r| r.f(controllers::get_obstacles))
            .resource("/get/lines", |r| r.f(controllers::get_lines))
            .resource("/get/distance", |r| r.f(controllers::get_distance))
    })
    .bind("0.0.0.0:80")
    .unwrap()
    .run();

    Ok(())
}
