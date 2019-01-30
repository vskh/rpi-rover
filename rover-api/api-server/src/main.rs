extern crate actix_web;

use actix_web::{server, App};

mod controllers;

fn main() {
    println!("Starting API server on port 8080.");
    server::new(|| App::new().resource("/", |r| r.f(controllers::index)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
