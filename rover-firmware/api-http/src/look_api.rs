use actix_web::{get, post, Responder, web};
use log::{debug, trace};
use serde::Deserialize;

use crate::app;
use crate::app::map_rover_status_to_response;
use libdriver::api::AsyncLooker;

#[derive(Deserialize)]
pub struct LookRequest {
    h: i16,
    v: i16,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(look_state)
        .service(look_at);
}

#[get("/")]
pub async fn look_state() -> impl Responder {
    "Current look state: unknown"
}

#[post("/")]
pub async fn look_at(req: web::Json<LookRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to look at ({}, {})", req.h, req.v);

    let r = map_rover_status_to_response(state.rover_client.lock().await.look_at(req.h, req.v).await);

    trace!("Returning {:#?}", r);

    r
}