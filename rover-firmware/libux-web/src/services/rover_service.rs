use std::fmt::Debug;
use serde::Deserialize;
use anyhow::{anyhow, Error};
use log::{trace, warn};
use yew::Callback;
use yew::format::{Json, Text, Nothing};
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use libapi_http::api::{MoveRequest, MoveType, LookRequest, SenseType, ValueResponse};

pub struct RoverService {
    rover_api_endpoint: String
}

type Status = std::result::Result<(), Error>;

impl RoverService {
    pub fn new(endpoint: &str) -> Self {
        RoverService {
            rover_api_endpoint: endpoint.to_owned()
        }
    }

    pub fn r#move(&self, r#type: MoveType, speed: u8, oncomplete: Callback<Status>) -> Result<FetchTask, Error> {
        let api_endpoint = format!("{}/move", self.rover_api_endpoint);
        let data = MoveRequest { r#type, speed };
        let r = Request::post(api_endpoint)
            .header("Content-Type", "application/json")
            .body(Json(&data))?;

        let handler = move |response: Response<Text>| {
            let (meta, body) = response.into_parts();

            if meta.status.is_success() {
                trace!("Move request completed successfully.");
                oncomplete.emit(body.map(|_| ()));
            } else {
                warn!("Move request failed: [{}] {:#?}", meta.status, body);
                oncomplete.emit(Err(anyhow!("Move operation failed: [{}] {:?}", meta.status, body)));
            }
        };

        FetchService::fetch(r, handler.into())
    }

    pub fn look_at(&self, h: i16, v: i16, oncomplete: Callback<Status>) -> Result<FetchTask, Error> {
        let api_endpoint = format!("{}/look", self.rover_api_endpoint);
        let data = LookRequest { h, v };
        let r = Request::post(api_endpoint)
            .header("Content-Type", "application/json")
            .body(Json(&data))?;

        let handler = move |response: Response<Text>| {
            let (meta, body) = response.into_parts();

            if meta.status.is_success() {
                trace!("Look request completed successfully.");
                oncomplete.emit(body.map(|_| ()));
            } else {
                warn!("Look request failed: [{}] {:#?}", meta.status, body);
                oncomplete.emit(Err(anyhow!("Look operation failed: [{}] {:?}", meta.status, body)));
            }
        };

        FetchService::fetch(r, handler.into())
    }

    pub fn get_distance(&self, oncomplete: Callback<Result<f32, Error>>) -> Result<FetchTask, Error> {
        self.sense(SenseType::Distance, oncomplete)
    }

    pub fn get_lines(&self, oncomplete: Callback<Result<Vec<bool>, Error>>) -> Result<FetchTask, Error> {
        self.sense(SenseType::Lines, oncomplete)
    }

    pub fn get_obstacles(&self, oncomplete: Callback<Result<Vec<bool>, Error>>) -> Result<FetchTask, Error> {
        self.sense(SenseType::Obstacles, oncomplete)
    }

    fn sense<T: 'static>(&self, r#type: SenseType, oncomplete: Callback<Result<T, Error>>) -> Result<FetchTask, Error>
        where T: Debug + for <'de> Deserialize<'de> {
        let api_endpoint = match r#type {
            SenseType::Distance => format!("{}/sense/distance", self.rover_api_endpoint),
            SenseType::Lines => format!("{}/sense/lines", self.rover_api_endpoint),
            SenseType::Obstacles => format!("{}/sense/obstacles", self.rover_api_endpoint)
        };

        let r = Request::get(api_endpoint)
            .header("Content-Type", "application/json")
            .body(Nothing)?;

        let handler = move |response: Response<Json<Result<ValueResponse<T>, Error>>>| {
            let (meta, Json(body)) = response.into_parts();

            if meta.status.is_success() {
                trace!("Sense '{}' result: {:#?}", r#type, body);
                oncomplete.emit(body.map(|vr| vr.value));
            } else {
                warn!("Sense '{}' request failed: [{}] {:#?}", r#type, meta.status, body);
                oncomplete.emit(Err(anyhow!("Sense operation failed: [{}] {:?}", meta.status, body)));
            }
        };

        FetchService::fetch(r, handler.into())
    }
}