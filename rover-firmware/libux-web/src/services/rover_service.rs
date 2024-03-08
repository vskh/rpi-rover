use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future;
use std::rc::Rc;

use anyhow::anyhow;
use gloo_net::http::Request;
use log::{error, trace, warn};
use serde::Deserialize;
use web_sys::wasm_bindgen::JsValue;
use web_sys::AbortController;
use yew::platform::spawn_local;
use yew::Callback;

use libapi_http::api::{LookRequest, MoveRequest, MoveType, SenseType, ValueResponse};

const REQUEST_SENSORS_TASK: &str = "task/timeout/request_sensors";
const GET_DISTANCE_TASK: &str = "task/sense/distance";
const GET_LINES_TASK: &str = "task/sense/lines";
const GET_OBSTACLES_TASK: &str = "task/sense/obstacles";

const MOVE_TASK: &str = "task/move";
const LOOK_TASK: &str = "task/look";

pub struct RoverService {
    rover_api_endpoint: String,
    pending_requests: Rc<RefCell<HashMap<&'static str, Rc<AbortController>>>>,
}

type RoverServiceError = anyhow::Error;

type Status = Result<(), RoverServiceError>;

impl RoverService {
    pub fn new(endpoint: &str) -> Self {
        RoverService {
            rover_api_endpoint: endpoint.to_owned(),
            pending_requests: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn map_jsvalue_err(value: JsValue) -> RoverServiceError {
        anyhow!("JsError: {:?}", value)
    }

    fn map_gloo_err(value: gloo_net::Error) -> RoverServiceError {
        value.into()
    }

    pub fn r#move(
        &self,
        r#type: MoveType,
        speed: u8,
        oncomplete: Callback<Status>,
    ) -> Result<Rc<AbortController>, RoverServiceError> {
        // cancel previous backend request, if any
        loop {
            match self.pending_requests.try_borrow_mut() {
                Ok(mut requests) => {
                    match requests.remove(MOVE_TASK) {
                        Some(pending_controller) => {
                            pending_controller.abort();
                        }
                        _ => {}
                    }

                    break;
                }
                _ => {}
            }
            trace!("Attempting to record new pending request...");
        }

        // prepare abort controller for the new request
        let controller = Rc::new(AbortController::new().map_err(Self::map_jsvalue_err)?);
        let signal = controller.signal();

        // prepare request
        let api_endpoint = format!("{}/move", self.rover_api_endpoint);
        let data = MoveRequest { r#type, speed };
        let req = Request::post(&api_endpoint)
            .abort_signal(Some(&signal))
            .json(&data)
            .map_err(Self::map_gloo_err)?;

        // launch async task
        // any error within async scope cannot propagate directly to method return and
        // should be communicated via passed callback
        {
            let controller = controller.clone();
            let pending_requests = self.pending_requests.clone();

            pending_requests.borrow_mut().insert(MOVE_TASK, controller);

            spawn_local(async move {
                trace!("Spawning async move request: {:?}", req);

                // send request
                let result = req.send().await;

                // provide back the response
                match result {
                    Ok(res) => {
                        trace!("Successfully obtained response to move request: {:?}", res);
                        if res.ok() {
                            trace!("Move request succeeded.");
                            oncomplete
                                .emit(res.text().await.map_or_else(|e| Err(e.into()), |_| Ok(())));
                        } else {
                            let response_body = res.text().await;
                            warn!(
                                "Move request failed: [{}] {:#?}",
                                res.status(),
                                response_body
                            );
                            oncomplete.emit(Err(anyhow!(
                                "Move direction update failed: [{}] {:?}",
                                res.status(),
                                response_body
                            )));
                        }
                    }
                    Err(e) => {
                        error!("Failed to send move request: {:?}", e);

                        match e {
                            gloo_net::Error::JsError(js_error) if js_error.name == "AbortError" => {
                                /* ignore self-inflicted error caused by us aborting previous request */
                            }
                            _ => oncomplete.emit(Err(Self::map_gloo_err(e))),
                        };
                    }
                }

                loop {
                    match pending_requests.try_borrow_mut() {
                        Ok(mut requests) => {
                            requests.remove(MOVE_TASK);
                            break;
                        }
                        _ => {}
                    }
                    trace!("Attempting delete record of completed request...");
                }
            });
        }

        // return abort controller for the spawned request
        Ok(controller)
    }

    // pub fn look_at(
    //     &self,
    //     h: i16,
    //     v: i16,
    //     oncomplete: Callback<Status>,
    // ) -> Result<FetchTask, Error> {
    //     let api_endpoint = format!("{}/look", self.rover_api_endpoint);
    //     let data = LookRequest { h, v };
    //     let r = Request::post(api_endpoint)
    //         .header("Content-Type", "application/json")
    //         .body(Json(&data))?;
    //
    //     let handler = move |response: Response<Text>| {
    //         let (meta, body) = response.into_parts();
    //
    //         if meta.status.is_success() {
    //             trace!("Look request completed successfully.");
    //             oncomplete.emit(body.map(|_| ()));
    //         } else {
    //             warn!("Look request failed: [{}] {:#?}", meta.status, body);
    //             oncomplete.emit(Err(anyhow!(
    //                 "Look operation failed: [{}] {:?}",
    //                 meta.status,
    //                 body
    //             )));
    //         }
    //     };
    //
    //     FetchService::fetch(r, handler.into())
    // }
    //
    // pub fn get_distance(
    //     &self,
    //     oncomplete: Callback<Result<f32, Error>>,
    // ) -> Result<FetchTask, Error> {
    //     self.sense(SenseType::Distance, oncomplete)
    // }
    //
    // pub fn get_lines(
    //     &self,
    //     oncomplete: Callback<Result<Vec<bool>, Error>>,
    // ) -> Result<FetchTask, Error> {
    //     self.sense(SenseType::Lines, oncomplete)
    // }
    //
    // pub fn get_obstacles(
    //     &self,
    //     oncomplete: Callback<Result<Vec<bool>, Error>>,
    // ) -> Result<FetchTask, Error> {
    //     self.sense(SenseType::Obstacles, oncomplete)
    // }
    //
    // fn sense<T: 'static>(
    //     &self,
    //     r#type: SenseType,
    //     oncomplete: Callback<Result<T, Error>>,
    // ) -> Result<FetchTask, Error>
    // where
    //     T: Debug + for<'de> Deserialize<'de>,
    // {
    //     let api_endpoint = match r#type {
    //         SenseType::Distance => format!("{}/sense/distance", self.rover_api_endpoint),
    //         SenseType::Lines => format!("{}/sense/lines", self.rover_api_endpoint),
    //         SenseType::Obstacles => format!("{}/sense/obstacles", self.rover_api_endpoint),
    //     };
    //
    //     let r = Request::get(api_endpoint)
    //         .header("Content-Type", "application/json")
    //         .body(Nothing)?;
    //
    //     let handler = move |response: Response<Json<Result<ValueResponse<T>, Error>>>| {
    //         let (meta, Json(body)) = response.into_parts();
    //
    //         if meta.status.is_success() {
    //             trace!("Sense '{}' result: {:#?}", r#type, body);
    //             oncomplete.emit(body.map(|vr| vr.value));
    //         } else {
    //             warn!(
    //                 "Sense '{}' request failed: [{}] {:#?}",
    //                 r#type, meta.status, body
    //             );
    //             oncomplete.emit(Err(anyhow!(
    //                 "Sense operation failed: [{}] {:?}",
    //                 meta.status,
    //                 body
    //             )));
    //         }
    //     };
    //
    //     FetchService::fetch(r, handler.into())
    // }
}
