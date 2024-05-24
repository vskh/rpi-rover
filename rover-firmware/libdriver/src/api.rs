use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::RoverError;

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveType {
    Forward,
    Backward,
    SpinCW,
    SpinCCW,
}

#[derive(Copy, Clone, Debug)]
pub struct RoverMoveDirection {
    l: i16,
    r: i16,
}

impl RoverMoveDirection {
    pub fn new(init_direction: (i16, i16)) -> Self {
        RoverMoveDirection {
            l: init_direction.0,
            r: init_direction.1
        }
    }

    pub fn get_raw_motors_speed(&self) -> (i16, i16) {
        (self.l, self.r)
    }

    pub fn get_direction(&self) -> Option<MoveType> {
        if self.l == 0 && self.r == 0 {
            None
        } else {
            Some(match (self.l > 0, self.r > 0, self.l == self.r) {
                (true, true, _) => MoveType::Forward,
                (false, false, _) => MoveType::Backward,
                (true, false, _) => MoveType::SpinCW,
                (false, true, _) => MoveType::SpinCCW,
            })
        }
    }

    pub fn get_speed(&self) -> u8 {
        match self.get_direction() {
            None => 0,
            Some(mt) => match mt {
                MoveType::Forward | MoveType::Backward => ((self.l + self.r) / 2).abs() as u8,
                MoveType::SpinCW | MoveType::SpinCCW => ((self.l.abs() + self.r.abs()) / 2) as u8
            },
        }
    }

    pub fn update(&mut self, l: Option<i16>, r: Option<i16>) {
        if let Some(s) = l {
            self.l = s;
        }

        if let Some(s) = r {
            self.r = s;
        }
    }
}

pub trait Mover {
    type Error: RoverError;

    fn stop(&mut self) -> Result<(), Self::Error>;
    fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error>;
    fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error>;
    fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error>;
    fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error>;

    fn get_move_direction(&self) -> Result<RoverMoveDirection, Self::Error>;

    fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait Looker {
    type Error: RoverError;

    fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error>;

    fn get_look_direction(&self) -> Result<(i16, i16), Self::Error>;

    fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait Sensor {
    type Error: RoverError;

    fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error>;
    fn get_lines(&self) -> Result<Vec<bool>, Self::Error>;
    fn scan_distance(&mut self) -> Result<f32, Self::Error>;

    fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
pub trait AsyncMover {
    type Error: RoverError;

    async fn stop(&mut self) -> Result<(), Self::Error>;
    async fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error>;
    async fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error>;
    async fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error>;
    async fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error>;

    async fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
pub trait AsyncLooker {
    type Error: RoverError;

    async fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error>;

    async fn get_look_direction(&self) -> Result<(i16, i16), Self::Error>;

    async fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
pub trait AsyncSensor {
    type Error: RoverError;

    async fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error>;
    async fn get_lines(&self) -> Result<Vec<bool>, Self::Error>;
    async fn scan_distance(&mut self) -> Result<f32, Self::Error>;

    async fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
