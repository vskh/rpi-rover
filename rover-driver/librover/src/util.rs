use std::sync::{Arc, Mutex};

use crate::api::{Looker, Mover, Sensor};
use crate::Result;

// RaspberryPi model B+ physical pins to BCM map
#[allow(dead_code)]
const PIN_TO_GPIO_REV3: [i8; 41] = [
    -1, -1, -1, 2, -1, 3, -1, 4, 14, -1, 15, 17, 18, 27, -1, 22, 23, -1, 24, 10, -1, 9, 24, 11,
    7, -1, 7, -1, -1, 5, -1, 6, 12, 13, -1, 19, 16, 26, 20, -1, 21,
];

// RaspberryPi model B+ BCM to physical pins map
const GPIO_TO_PIN_REV3: [i8; 33] = [
    -1, -1, 3, 5, 7, 29, 31, 26, 24, 21, 19, 23, 32, 33, 8, 10, 36, 11, 12, 35, 38, 40, 15, 16,
    18, 22, 37, 13, -1, -1, -1, -1, 0,
];

pub fn bcm2pin(gpio_id: u8) -> i8 {
    GPIO_TO_PIN_REV3[gpio_id as usize]
}

pub fn pin2bcm(pin_id: u8) -> i8 {
    GPIO_TO_PIN_REV3[pin_id as usize]
}

pub struct MoverPart<'a, T>(Arc<Mutex<&'a mut T>>) where T: Mover;

impl<'a, T> Mover for MoverPart<'a, T> where T: Mover {
    fn stop(&mut self) -> Result<()> {
        let mut mover = self.0.lock().unwrap();
        mover.stop()
    }

    fn move_forward(&mut self, speed: u8) -> Result<()> {
        let mut mover = self.0.lock().unwrap();
        mover.move_forward(speed)
    }

    fn move_backward(&mut self, speed: u8) -> Result<()> {
        let mut mover = self.0.lock().unwrap();
        mover.move_backward(speed)
    }

    fn spin_right(&mut self, speed: u8) -> Result<()> {
        let mut mover = self.0.lock().unwrap();
        mover.spin_right(speed)
    }

    fn spin_left(&mut self, speed: u8) -> Result<()> {
        let mut mover = self.0.lock().unwrap();
        mover.spin_left(speed)
    }
}

pub struct LookerPart<'a, T>(Arc<Mutex<&'a mut T>>) where T: Looker;

impl<'a, T> Looker for LookerPart<'a, T> where T: Looker {
    fn look_at(&mut self, h: i16, v: i16) -> Result<()> {
        let mut looker = self.0.lock().unwrap();
        looker.look_at(h, v)
    }
}

pub struct SensorPart<'a, T>(Arc<Mutex<&'a mut T>>) where T: Sensor;

impl<'a, T> Sensor for SensorPart<'a, T> where T: Sensor {
    fn get_obstacles(&self) -> Result<Vec<bool>> {
        let sensor = self.0.lock().unwrap();
        sensor.get_obstacles()
    }

    fn get_lines(&self) -> Result<Vec<bool>> {
        let sensor = self.0.lock().unwrap();
        sensor.get_lines()
    }

    fn get_distance(&mut self) -> Result<f32> {
        let mut sensor = self.0.lock().unwrap();
        sensor.get_distance()
    }
}

pub trait SplittableRover where Self: Sized + Mover + Looker + Sensor {
    fn split(&mut self) -> (MoverPart<Self>, LookerPart<Self>, SensorPart<Self>) {
        let l = Arc::new(Mutex::new(self));

        (
            MoverPart(Arc::clone(&l)),
            LookerPart(Arc::clone(&l)),
            SensorPart(Arc::clone(&l))
        )
    }
}
