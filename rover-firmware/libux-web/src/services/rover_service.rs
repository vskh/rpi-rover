use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future;
use std::rc::Rc;

use anyhow::anyhow;
use gloo_net::http::Request;
use log::{error, trace, warn};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsValue;
use web_sys::AbortController;
use yew::platform::spawn_local;
use yew::Callback;

use libapi_http::api::{LookRequest, MoveRequest, MoveType, SenseType, ValueResponse};
use libutil::helpers::calc_hash;

pub struct RoverService {
    rover_api_endpoint: String,
    pending_requests: Rc<RefCell<HashMap<u64, Rc<AbortController>>>>,
}

type RoverServiceError = anyhow::Error;

type Status<T = ()> = Result<T, RoverServiceError>;
type PendingStatus = Result<Rc<AbortController>, RoverServiceError>;

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
    ) -> PendingStatus {
        let api_endpoint = format!("{}/move", self.rover_api_endpoint);
        let data = MoveRequest { r#type, speed };

        self.schedule_request(&api_endpoint, &data, oncomplete)
    }

    pub fn look_at(
        &self,
        h: i16,
        v: i16,
        oncomplete: Callback<Status>,
    ) -> PendingStatus {
        let api_endpoint = format!("{}/look", self.rover_api_endpoint);
        let data = LookRequest { h, v };

        self.schedule_request(&api_endpoint, &data, oncomplete)
    }

    pub fn get_distance(
        &self,
        oncomplete: Callback<Status<f32>>,
    ) -> PendingStatus {
        self.sense(SenseType::Distance, oncomplete)
    }

    pub fn get_lines(
        &self,
        oncomplete: Callback<Status<Vec<bool>>>,
    ) -> PendingStatus {
        self.sense(SenseType::Lines, oncomplete)
    }

    pub fn get_obstacles(
        &self,
        oncomplete: Callback<Status<Vec<bool>>>,
    ) -> PendingStatus {
        self.sense(SenseType::Obstacles, oncomplete)
    }

    fn sense<T>(
        &self,
        r#type: SenseType,
        oncomplete: Callback<Status<T>>,
    ) -> PendingStatus
    where
        T: Debug + DeserializeOwned + 'static,
    {
        let api_endpoint = match r#type {
            SenseType::Distance => format!("{}/sense/distance", self.rover_api_endpoint),
            SenseType::Lines => format!("{}/sense/lines", self.rover_api_endpoint),
            SenseType::Obstacles => format!("{}/sense/obstacles", self.rover_api_endpoint),
        };

        self.schedule_request(&api_endpoint, &(), oncomplete)
    }

    fn schedule_request<TRequest, TResponse>(
        &self,
        api_endpoint: &str,
        request_data: &TRequest,
        oncomplete: Callback<Result<TResponse, RoverServiceError>>,
    ) -> Result<Rc<AbortController>, RoverServiceError>
    where
        TRequest: Serialize,
        TResponse: Debug + DeserializeOwned + 'static,
    {
        let api_endpoint_id = calc_hash(api_endpoint);

        // cancel previous backend request, if any
        loop {
            match self.pending_requests.try_borrow_mut() {
                Ok(mut requests) => {
                    match requests.remove(&api_endpoint_id) {
                        Some(pending_controller) => {
                            pending_controller.abort();
                        }
                        _ => {}
                    }

                    break;
                }
                _ => {}
            }
            trace!("Attempting to record a pending {{{}}} request...", api_endpoint);
        }

        // prepare abort controller for the new request
        let controller = Rc::new(AbortController::new().map_err(Self::map_jsvalue_err)?);
        let signal = controller.signal();

        // prepare request
        let req = Request::post(api_endpoint)
            .abort_signal(Some(&signal))
            .json(request_data)
            .map_err(Self::map_gloo_err)?;

        // launch async task
        // any error within async scope cannot propagate directly to method return and
        // should be communicated via passed callback
        {
            let controller = controller.clone();
            let pending_requests = self.pending_requests.clone();
            let api_endpoint = api_endpoint.to_owned();

            pending_requests.borrow_mut().insert(api_endpoint_id, controller);

            spawn_local(async move {
                trace!("Spawning async {{{}}} request: {:?}", api_endpoint, req);

                // send request
                let result = req.send().await;

                // provide back the response
                match result {
                    Ok(res) => {
                        trace!("Successfully obtained response to {{{}}} request: {:?}", api_endpoint, res);
                        if res.ok() {
                            trace!("{{{}}} request succeeded.", api_endpoint);
                            oncomplete
                                .emit(res.text().await.and_then(|mut text| {
                                    if text.is_empty() {
                                        text += "null";
                                    }

                                    serde_json::from_str::<TResponse>(&text).map_err(gloo_net::Error::from)
                                }).map_err(|e| e.into()))
                        } else {
                            let response_body = res.text().await;
                            warn!(
                                "{{{}}} request failed: [{}] {:#?}",
                                api_endpoint,
                                res.status(),
                                response_body
                            );
                            oncomplete.emit(Err(anyhow!(
                                "{{{}}} failed: [{}] {:?}",
                                api_endpoint,
                                res.status(),
                                response_body
                            )));
                        }
                    }
                    Err(e) => {
                        error!("Failed to send {{{}}} request: {:?}", api_endpoint, e);

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
                            requests.remove(&api_endpoint_id);
                            break;
                        }
                        _ => {}
                    }
                    trace!("Attempting delete record of completed {{{}}} request...", api_endpoint);
                }
            });
        }

        // return abort controller for the spawned request
        Ok(controller)
    }
}
