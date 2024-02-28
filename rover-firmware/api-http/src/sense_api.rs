use actix_web::{get, web, Responder};
use log::{debug, trace};

use libdriver::api::AsyncSensor;

use crate::app;
use crate::app::map_rover_result_to_response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_obstacles)
        .service(get_lines)
        .service(get_distance);
}

#[get("/obstacles")]
pub async fn get_obstacles(state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to provide obstacles data.");

    let r = map_rover_result_to_response(state.rover_client.lock().await.get_obstacles().await);

    trace!("Returning {:#?}", r);

    r
}

#[get("/lines")]
pub async fn get_lines(state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to provide lines data.");

    let r = map_rover_result_to_response(state.rover_client.lock().await.get_lines().await);

    trace!("Returning {:#?}", r);

    r
}

#[get("/distance")]
pub async fn get_distance(state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to provide sonar distance.");

    let r = map_rover_result_to_response(state.rover_client.lock().await.scan_distance().await);

    trace!("Returning {:#?}", r);

    r
}
