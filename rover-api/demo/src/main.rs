extern crate rover;
extern crate robohat;

use std::{thread, time};

use rover::api::Rover;
use robohat::RobohatRover;

fn main() {
    println!("Initializing rover");
    let mut rover = RobohatRover::new().unwrap();
    println!("Starting rover...");

    println!("Moving forward");
    rover.move_forward(0.5);
    thread::sleep(time::Duration::from_secs(1));
    println!("Moving forward faster");
    rover.move_forward(1.0);
    thread::sleep(time::Duration::from_secs(1));
    println!("Spinning left");
    rover.spin_left(0.25);
    thread::sleep(time::Duration::from_secs(1));
    println!("Moving backward");
    rover.move_backward(0.5);
    thread::sleep(time::Duration::from_secs(1));
    println!("Moving backward faster");
    rover.move_backward(1.0);
    thread::sleep(time::Duration::from_secs(1));
    println!("Spinning right");
    rover.spin_right(0.75);
    thread::sleep(time::Duration::from_secs(1));
    println!("Stopping");
    rover.stop();

    println!("Finished!");
}
