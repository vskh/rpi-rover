use serde::{Deserialize, Serialize};
use strum_macros::Display as EnumDisplay;

#[derive(Debug, EnumDisplay, Serialize, Deserialize)]
pub enum MoveType {
    Forward,
    Backward,
    CWSpin,
    CCWSpin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveRequest {
    pub r#type: MoveType,
    pub speed: u8,
}

#[derive(Debug, EnumDisplay, Serialize, Deserialize)]
pub enum SenseType {
    Lines,
    Obstacles,
    Distance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookRequest {
    pub h: i16,
    pub v: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueResponse<T> {
    pub value: T,
}
