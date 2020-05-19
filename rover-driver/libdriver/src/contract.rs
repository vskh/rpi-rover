use async_trait::async_trait;
use crate::Result;

pub mod data {
    use serde::{Serialize, Deserialize};


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
        pub(crate) move_type: MoveType,
        pub(crate) speed: u8
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LookRequest {
        pub(crate) x: i16,
        pub(crate) y: i16
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
        Success,
        Error(String)
    }
}

#[async_trait]
pub trait Mover {
    async fn stop(&mut self) -> Result<()>;
    async fn move_forward(&mut self, speed: u8) -> Result<()>;
    async fn move_backward(&mut self, speed: u8) -> Result<()>;
    async fn spin_right(&mut self, speed: u8) -> Result<()>;
    async fn spin_left(&mut self, speed: u8) -> Result<()>;
}

#[async_trait]
pub trait Looker {
    async fn look_at(&mut self, h: i16, v: i16) -> Result<()>;
}

#[async_trait]
pub trait Sensor {
    async fn get_obstacles(&self) -> Result<()>;
    async fn get_lines(&self) -> Result<()>;
    async fn get_distance(&mut self) -> Result<()>;
}
