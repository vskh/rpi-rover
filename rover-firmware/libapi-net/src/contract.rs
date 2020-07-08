pub mod data {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ProtocolMessage {
        MoveRequest(MoveRequest),
        LookRequest(LookRequest),
        SenseRequest(SenseRequest),
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
        Obstacle,
        Line,
        Distance
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SenseResponse {
        Obstacle(Vec<bool>),
        Line(Vec<bool>),
        Distance(f32),
        Error(String)
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum StatusResponse {
        Success,
        Error(String)
    }
}

