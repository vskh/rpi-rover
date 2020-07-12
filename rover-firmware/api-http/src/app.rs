use libapi_net::client::Client;
use futures::lock::Mutex;
use actix_web::HttpResponse;
use serde::Serialize;

pub struct State {
    pub rover_client: Mutex<Client>
}

#[derive(Debug, Serialize)]
struct ResultResponse<T> {
    result: T
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
            .json(ResultResponse { result: v }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body(e.to_string())
    }
}