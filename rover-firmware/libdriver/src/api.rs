use futures::executor::block_on;

use async_trait::async_trait;

use crate::RoverError;

pub trait Mover {
    type Error: RoverError;

    fn stop(&mut self) -> Result<(), Self::Error>;
    fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error>;
    fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error>;
    fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error>;
    fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error>;
}

pub trait Looker {
    type Error: RoverError;

    fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error>;
}

pub trait Sensor {
    type Error: RoverError;

    fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error>;
    fn get_lines(&self) -> Result<Vec<bool>, Self::Error>;
    fn scan_distance(&mut self) -> Result<f32, Self::Error>;
}

#[async_trait]
pub trait AsyncMover {
    type Error: RoverError;

    async fn stop(&mut self) -> Result<(), Self::Error>;
    async fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error>;
    async fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error>;
    async fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error>;
    async fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait AsyncLooker {
    type Error: RoverError;

    async fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait AsyncSensor {
    type Error: RoverError;

    async fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error>;
    async fn get_lines(&self) -> Result<Vec<bool>, Self::Error>;
    async fn scan_distance(&mut self) -> Result<f32, Self::Error>;
}

impl<T> Mover for T where T: AsyncMover {
    type Error = T::Error;

    fn stop(&mut self) -> Result<(), Self::Error> {
        block_on(AsyncMover::stop(self))
    }

    fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error> {
        block_on(AsyncMover::move_forward(self, speed))
    }

    fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error> {
        block_on(AsyncMover::move_backward(self, speed))
    }

    fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error> {
        block_on(AsyncMover::spin_right(self, speed))
    }

    fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error> {
        block_on(AsyncMover::spin_left(self, speed))
    }
}

impl<T> Looker for T where T: AsyncLooker {
    type Error = T::Error;

    fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error> {
        block_on(AsyncLooker::look_at(self, h, v))
    }
}

impl<T> Sensor for T where T: AsyncSensor {
    type Error = T::Error;

    fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error> {
        block_on(AsyncSensor::get_obstacles(self))
    }

    fn get_lines(&self) -> Result<Vec<bool>, Self::Error> {
        block_on(AsyncSensor::get_lines(self))
    }

    fn scan_distance(&mut self) -> Result<f32, Self::Error> {
        block_on(AsyncSensor::scan_distance(self))
    }
}