use yew::Callback;
use yew::format::{Json, Text, Nothing};
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use libapi_http::api::{MoveRequest, MoveType, LookRequest, SenseType, SenseRequest};

pub struct RoverService {
    rover_api_endpoint: String
}

impl RoverService {
    pub fn new(endpoint: &str) -> Self {
        RoverService {
            rover_api_endpoint: endpoint.to_owned()
        }
    }

    pub fn r#move(&self, r#type: MoveType, speed: u8, cb: Callback<Response<Text>>) -> Result<FetchTask, anyhow::Error> {
        let api_endpoint = format!("{}/move", self.rover_api_endpoint);
        let data = MoveRequest { r#type, speed };
        let r = Request::post(api_endpoint)
            .header("Content-Type", "application/json")
            .body(Json(&data))?;

        FetchService::fetch(r, cb)
    }

    pub fn look(&self, h: i16, v: i16, cb: Callback<Response<Text>>) -> Result<FetchTask, anyhow::Error> {
        let api_endpoint = format!("{}/look", self.rover_api_endpoint);
        let data = LookRequest { h, v };
        let r = Request::post(api_endpoint)
            .header("Content-Type", "application/json")
            .body(Json(&data))?;

        FetchService::fetch(r, cb)
    }

    pub fn sense(&self, r#type: SenseType, cb: Callback<Response<Text>>) -> Result<FetchTask, anyhow::Error> {
        let api_endpoint = match r#type {
            SenseType::Distance => format!("{}/sense/distance", self.rover_api_endpoint),
            SenseType::Lines => format!("{}/sense/lines", self.rover_api_endpoint),
            SenseType::Obstacles => format!("{}/sense/obstacles", self.rover_api_endpoint)
        };

        let r = Request::get(api_endpoint)
            .header("Content-Type", "application/json")
            .body(Nothing)?;

        FetchService::fetch(r, cb)
    }
}