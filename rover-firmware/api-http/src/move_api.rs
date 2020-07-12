use actix_web::{get, post, Responder, web};
use log::{debug, trace};
use serde::Deserialize;

use libdriver::api::AsyncMover;

use crate::app;
use crate::app::map_rover_status_to_response;

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    speed: u8,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(move_state)
        .service(move_forward)
        .service(move_backward)
        .service(spin_left)
        .service(spin_right);
}

#[get("/")]
pub async fn move_state() -> impl Responder {
    "Current move state: unknown"
}

#[post("/forward")]
pub async fn move_forward(req: web::Json<MoveRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to move forward with {} speed", req.speed);

    let r = map_rover_status_to_response(state.rover_client.lock().await.move_forward(req.speed).await);

    trace!("Returning {:#?}", r);

    r
}

#[post("/backward")]
pub async fn move_backward(req: web::Json<MoveRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to move backward with {} speed", req.speed);

    let r = map_rover_status_to_response(state.rover_client.lock().await.move_backward(req.speed).await);

    trace!("Returning {:#?}", r);

    r
}

#[post("/ccw")]
pub async fn spin_left(req: web::Json<MoveRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to spin left with speed {}", req.speed);

    let r = map_rover_status_to_response(state.rover_client.lock().await.spin_left(req.speed).await);

    trace!("Returning {:#?}", r);

    r
}

#[post("/cw")]
pub async fn spin_right(req: web::Json<MoveRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to spin right with speed {}", req.speed);

    let r = map_rover_status_to_response(state.rover_client.lock().await.spin_right(req.speed).await);

    trace!("Returning {:#?}", r);

    r
}
