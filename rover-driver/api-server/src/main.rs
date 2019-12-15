extern crate actix_web;

use actix_web::{server, App};

mod controllers;

fn main() {
    println!("Starting API server on port 8080.");
    server::new(|| {
        App::new()
            .resource("/move/forward", |r| r.f(controllers::move_forward))
            .resource("/move/backward", |r| r.f(controllers::move_backward))
            .resource("/spin/left", |r| r.f(controllers::spin_left))
            .resource("/spin/right", |r| r.f(controllers::spin_right))
            .resource("/look", |r| r.f(controllers::look))
            .resource("/get/obstacles", |r| r.f(controllers::get_obstacles))
            .resource("/get/lines", |r| r.f(controllers::get_lines))
            .resource("/get/distance", |r| r.f(controllers::get_distance))
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run();
}
