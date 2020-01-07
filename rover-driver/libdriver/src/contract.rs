pub mod data {
    use serde::{Serialize, Deserialize};
    use serde::export::Formatter;
    use serde::export::fmt::Error;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ProtocolMessage {
        MoveRequest(MoveRequest),
        LookRequest(LookRequest),
        SenseRequest(SenseRequest),
        SenseSubscribeRequest(SenseSubscribeRequest),
        SenseResponse(SenseResponse),
        StatusResponse(StatusResponse)
    }

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
    pub enum StatusResponse {
        Ok,
        Error(String)
    }
}

pub trait Mover {
    fn stop(&mut self) -> data::StatusResponse;
    fn move_forward(&mut self, speed: u8) -> data::StatusResponse;
    fn move_backward(&mut self, speed: u8) -> data::StatusResponse;
    fn spin_right(&mut self, speed: u8) -> data::StatusResponse;
    fn spin_left(&mut self, speed: u8) -> data::StatusResponse;
}

pub trait Looker {
    fn look_at(&mut self, h: i16, v: i16) -> data::StatusResponse;
}

pub trait Sensor {
    fn get_obstacles(&self) -> data::SenseResponse;
    fn get_lines(&self) -> data::SenseResponse;
    fn get_distance(&mut self) -> data::SenseResponse;
}

pub trait Driver : Mover + Looker + Sensor {

}