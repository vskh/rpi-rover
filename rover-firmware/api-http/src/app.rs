use actix_web::HttpResponse;
use futures::lock::Mutex;
use serde::Serialize;

use libapi_net::client::Client;
use libapi_http::api::ValueResponse;

pub struct State {
    pub rover_client: Mutex<Client>
}

pub fn map_rover_status_to_response<T, E: std::error::Error>(r: Result<T, E>) -> HttpResponse {
    match r {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body(e.to_string())
    }
}

pub fn map_rover_result_to_response<T: Serialize, E: std::error::Error>(r: Result<T, E>) -> HttpResponse {
    match r {
        Ok(v) => HttpResponse::Ok()
            .content_type("application/json")
            .json(ValueResponse { value: v }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body(e.to_string())
    }
}