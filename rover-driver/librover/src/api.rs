use crate::Result;

pub trait Mover {
    fn stop(&mut self) -> Result<()>;
    fn move_forward(&mut self, speed: u8) -> Result<()>;
    fn move_backward(&mut self, speed: u8) -> Result<()>;
    fn spin_right(&mut self, speed: u8) -> Result<()>;
    fn spin_left(&mut self, speed: u8) -> Result<()>;
}

pub trait Looker {
    fn look_at(&mut self, h: i16, v: i16) -> Result<()>;
}

pub trait Sensor {
    fn get_obstacles(&self) -> Result<Vec<bool>>;
    fn get_lines(&self) -> Result<Vec<bool>>;
    fn get_distance(&mut self) -> Result<f32>;
}