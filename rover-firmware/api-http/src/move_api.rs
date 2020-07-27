use actix_web::{post, Responder, web};
use log::{debug, trace};

use libapi_http::api::{MoveRequest, MoveType};
use libdriver::api::AsyncMover;

use crate::app;
use crate::app::map_rover_status_to_response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(move_control);
}

#[post("")]
pub async fn move_control(req: web::Json<MoveRequest>, state: web::Data<app::State>) -> impl Responder {
    debug!("Requested to move {:#?} with speed of {}", req.r#type, req.speed);

    let mut client = state.rover_client.lock().await;
    let result = match req.r#type {
        MoveType::Forward => client.move_forward(req.speed),
        MoveType::Backward => client.move_backward(req.speed),
        MoveType::CWSpin => client.spin_right(req.speed),
        MoveType::CCWSpin => client.spin_left(req.speed)
    };

    let r = map_rover_status_to_response(result.await);

    trace!("Returning {:#?}", r);

    r
}
