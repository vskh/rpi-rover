pub mod data {
    use serde::{Deserialize, Serialize};
    use libdriver::api::MoveType;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ProtocolMessage {
        /// Request to move in given direction with given speed
        MoveRequest(MoveData),

        /// Request to look at given direction
        LookRequest(LookData),

        /// Request to see current look direction
        LookDirectionRequest,

        /// Response to the above
        LookDirectionResponse(LookData),

        /// Request to check the value of sensor
        SenseRequest(SenseRequestData),

        /// Response to the above
        SenseResponse(SenseResponseData),

        /// Response to requests that return no data (e.g. MoveRequest & LookRequest)
        StatusResponse(StatusResponseData),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MoveData {
        pub(crate) move_type: MoveType,
        pub(crate) speed: u8,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LookData {
        pub(crate) x: i16,
        pub(crate) y: i16,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SenseRequestData {
        Obstacle,
        Line,
        Distance,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SenseResponseData {
        Obstacle(Vec<bool>),
        Line(Vec<bool>),
        Distance(f32),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum StatusResponseData {
        Success,
        Error(String),
    }
}
