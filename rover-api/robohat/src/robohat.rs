use rppal::gpio::Gpio;
use rover::api;

pub struct RobohatRover {
    gpio: Gpio
}

impl api::Rover for RobohatRover {
    fn move_forward(&self, speed: &u32) {
//            self.gpio.
    }
}