use libdriver_robohat::RobohatRover;
use libridecontrol::controller::RideController;

fn main() {
    match RobohatRover::new() {
        Err(e) => println!("Rover initialization error: {}", e),
        Ok(rover) => match RideController::new(rover) {
            Err(e) => println!("Ride controller initialization error: {}", e),
            Ok(mut controller) => match controller.run() {
                Err(e) => println!("Rover issue: {}", e),
                Ok(()) => println!("Done.")
            }
        }
    }
}
