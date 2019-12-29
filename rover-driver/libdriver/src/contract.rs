pub mod data {
    use serde::{Serialize, Deserialize};
    use serde::export::Formatter;
    use serde::export::fmt::Error;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum MoveType {
        Forward,
        Backward,
        SpinCW,
        SpinCCW
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MoveRequest {
        move_type: MoveType,
        speed: u8
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LookRequest {
        x: i16,
        y: i16
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SenseRequest {
        Obstacle(u8),
        Line(u8),
        Distance
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SenseSubscribeRequest {
        on_change: bool,
        interval: u16
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SenseResponse {
        Obstacle(u8, bool),
        Line(u8, bool),
        Distance(f32)
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Response<T> {
        Ok(T),
        Error(String)
    }
}

pub trait Mover {
    fn stop(&mut self) -> data::Response<()>;
    fn move_forward(&mut self, speed: u8) -> data::Response<()>;
    fn move_backward(&mut self, speed: u8) -> data::Response<()>;
    fn spin_right(&mut self, speed: u8) -> data::Response<()>;
    fn spin_left(&mut self, speed: u8) -> data::Response<()>;
}

pub trait Looker {
    fn look_at(&mut self, h: i16, v: i16) -> data::Response<()>;
}

pub trait Sensor {
    fn get_obstacles(&self) -> data::Response<data::SenseResponse>;
    fn get_lines(&self) -> data::Response<data::SenseResponse>;
    fn get_distance(&mut self) -> data::Response<data::SenseResponse>;
}

pub trait Driver : Mover + Looker + Sensor {

}