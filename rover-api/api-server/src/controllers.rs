use actix_web::HttpRequest;

pub fn index(_req: &HttpRequest) -> &'static str {
    "RaspberryPi Rover"
}
