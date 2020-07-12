use actix_web::{get, post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
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
pub async fn move_forward(req: web::Json<MoveRequest>) -> impl Responder {
    format!("Moving forward with speed {}", req.speed)
}

#[post("/backward")]
pub async fn move_backward(req: web::Json<MoveRequest>) -> impl Responder {
    format!("Moving backward with speed {}", req.speed)
}

#[post("/ccw")]
pub async fn spin_left(req: web::Json<MoveRequest>) -> impl Responder {
    format!("Spinning left with speed {}", req.speed)
}

#[post("/cw")]
pub async fn spin_right(req: web::Json<MoveRequest>) -> impl Responder {
    format!("Spinning right with speed {}", req.speed)
}
