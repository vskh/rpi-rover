use actix_web::{get, post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LookRequest {
    h: i16,
    v: i16
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(look_state)
        .service(move_forward);
}

#[get("/")]
pub async fn look_state() -> impl Responder {
    "Current look state: unknown"
}

#[post("/")]
pub async fn move_forward(req: web::Json<LookRequest>) -> impl Responder {
    format!("Looking at {} {}", req.h, req.v)
}