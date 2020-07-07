use std::sync::{Arc, Mutex};

use crate::api::{Looker, Mover, Sensor};

pub struct MoverPart<'a, T>(Arc<Mutex<&'a mut T>>) where T: Mover;

impl<'a, T> Mover for MoverPart<'a, T> where T: Mover {
    type Error = T::Error;

    fn stop(&mut self) -> Result<(), T::Error> {
        let mut mover = self.0.lock().unwrap();
        mover.stop()
    }

    fn move_forward(&mut self, speed: u8) -> Result<(), T::Error> {
        let mut mover = self.0.lock().unwrap();
        mover.move_forward(speed)
    }

    fn move_backward(&mut self, speed: u8) -> Result<(), T::Error> {
        let mut mover = self.0.lock().unwrap();
        mover.move_backward(speed)
    }

    fn spin_right(&mut self, speed: u8) -> Result<(), T::Error> {
        let mut mover = self.0.lock().unwrap();
        mover.spin_right(speed)
    }

    fn spin_left(&mut self, speed: u8) -> Result<(), T::Error> {
        let mut mover = self.0.lock().unwrap();
        mover.spin_left(speed)
    }
}

pub struct LookerPart<'a, T>(Arc<Mutex<&'a mut T>>) where T: Looker;

impl<'a, T> Looker for LookerPart<'a, T> where T: Looker {
    type Error = T::Error;

    fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error> {
        let mut looker = self.0.lock().unwrap();
        looker.look_at(h, v)
    }
}

pub struct SensorPart<'a, T>(Arc<Mutex<&'a mut T>>) where T: Sensor;

impl<'a, T> Sensor for SensorPart<'a, T> where T: Sensor {
    type Error = T::Error;

    fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error> {
        let sensor = self.0.lock().unwrap();
        sensor.get_obstacles()
    }

    fn get_lines(&self) -> Result<Vec<bool>, Self::Error> {
        let sensor = self.0.lock().unwrap();
        sensor.get_lines()
    }

    fn scan_distance(&mut self) -> Result<f32, Self::Error> {
        let mut sensor = self.0.lock().unwrap();
        sensor.scan_distance()
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
