use actix_web::{post, Responder, web};
use log::{debug, trace};

use crate::app;
use crate::app::map_rover_status_to_response;
use libdriver::api::AsyncLooker;
use libapi_http::api::LookRequest;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(look_at);
}

#[post("")]
pub async fn look_at(req: web::Json<LookRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to look at ({}, {})", req.h, req.v);

    let r = map_rover_status_to_response(state.rover_client.lock().await.look_at(req.h, req.v).await);

    trace!("Returning {:#?}", r);

    r
}