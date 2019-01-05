extern crate rover;
extern crate robohat;

use std::{thread, time};

use rover::api::Rover;
use robohat::RobohatRover;

fn main() {
    println!("Initializing rover");
    let rover = RobohatRover::new().unwrap();
    println!("Starting rover...");
    
    rover.move_forward(10);
    thread::sleep(time::Duration::from_secs(3));
    rover.rotate_left(10);
    thread::sleep(time::Duration::from_secs(3));
    rover.move_backward(10);
    thread::sleep(time::Duration::from_secs(3));
    rover.rotate_right(10);
    thread::sleep(time::Duration::from_secs(3));
    rover.stop();

    println!("Finished!");
}
