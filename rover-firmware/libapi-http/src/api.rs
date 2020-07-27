use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveType {
    Forward,
    Backward,
    CWSpin,
    CCWSpin
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveRequest {
    pub r#type: MoveType,
    pub speed: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SenseType {
    Lines,
    Obstacles,
    Distance
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenseRequest {
    pub r#type: SenseType
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookRequest {
    pub h: i16,
    pub v: i16
}

#[derive(Debug, Serialize)]
pub struct ResultResponse<T> {
    pub result: T
}