use actix_web::{get, post, Responder, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_obstacles)
        .service(get_lines)
        .service(get_distance);
}

#[get("/obstacles")]
pub async fn get_obstacles() -> impl Responder {
    "Obstacles: none"
}

#[post("/lines")]
pub async fn get_lines() -> impl Responder {
    "Lines: none"
}

#[get("/distance")]
pub async fn get_distance() -> impl Responder {
    "Distance: unknown"
}