use std::rc::Rc;

use anyhow::{anyhow, Error};
use log::{debug, trace};
use stylist::yew::use_style;
use yew::prelude::*;

use libapi_http::api::MoveType;

use crate::components::direction_control::{
    DirectionControl, DirectionControlMode, DirectionModuleMode,
};
use crate::components::sensors_data::SensorsData;
use crate::services::rover_service::RoverService;

#[derive(Debug)]
pub enum AppAction {
    RequestSensors,
    SensorDirectionUpdate((i32, i32)),
    SensorDirectionUpdateError(Error, (i32, i32)),
    MoveDirectionUpdate((i32, i32)),
    MoveDirectionUpdateError(Error, (i32, i32)),
    DistanceUpdate(f32),
    DistanceUpdateError(Error),
    ObstaclesUpdate(Vec<bool>),
    ObstaclesUpdateError(Error),
    LinesUpdate(Vec<bool>),
    LinesUpdateError(Error),
}

#[derive(Default, Debug)]
pub struct AppState {
    pub sensor_direction: (i32, i32),
    pub sensor_direction_error: Rc<Option<Error>>,
    pub move_direction: (i32, i32),
    pub move_direction_error: Rc<Option<Error>>,
    pub distance: f32,
    pub distance_error: Rc<Option<Error>>,
    pub lines: Rc<Vec<bool>>,
    pub lines_error: Rc<Option<Error>>,
    pub obstacles: Rc<Vec<bool>>,
    pub obstacles_error: Rc<Option<Error>>,
}

impl AppState {
    fn select_move_type(&self) -> Option<MoveType> {
        if self.move_direction.1 > 0 {
            Some(MoveType::Forward)
        } else if self.move_direction.1 < 0 {
            Some(MoveType::Backward)
        } else if self.move_direction.0 > 0 {
            Some(MoveType::CWSpin)
        } else if self.move_direction.0 < 0 {
            Some(MoveType::CCWSpin)
        } else {
            None
        }
    }

    fn select_speed(&self) -> u8 {
        let unscaled_speed = if self.move_direction.0 != 0 {
            self.move_direction.0
        } else if self.move_direction.1 != 0 {
            self.move_direction.1
        } else {
            0
        };

        ((unscaled_speed.abs() as f64 / i32::MAX as f64) * (u8::MAX as f64)).floor() as u8
    }

    fn move_type_repr(&self) -> char {
        match self.select_move_type() {
            Some(MoveType::Forward) => '↑',
            Some(MoveType::Backward) => '↓',
            Some(MoveType::CWSpin) => '↻',
            Some(MoveType::CCWSpin) => '↺',
            None => '■',
        }
    }
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        trace!("Processing dispatched action: {:?}", action);

        let sensor_direction = match action {
            _ => self.sensor_direction,
        };
        let sensor_direction_error = match action {
            _ => self.sensor_direction_error.clone(),
        };

        let move_direction = match action {
            AppAction::MoveDirectionUpdate(dir) => dir,
            AppAction::MoveDirectionUpdateError(_, dir) => dir,
            _ => self.move_direction,
        };

        let move_direction_error = match action {
            AppAction::MoveDirectionUpdate(_) => None.into(),
            AppAction::MoveDirectionUpdateError(e, _) => Some(e).into(),
            _ => self.move_direction_error.clone(),
        };

        let distance = match action {
            _ => self.distance,
        };

        let distance_error = match action {
            _ => self.distance_error.clone(),
        };

        let lines = match action {
            _ => self.lines.clone(),
        };

        let lines_error = match action {
            _ => self.lines_error.clone(),
        };

        let obstacles = match action {
            _ => self.obstacles.clone(),
        };

        let obstacles_error = match action {
            _ => self.obstacles_error.clone(),
        };

        let new_state = Self {
            sensor_direction,
            sensor_direction_error,
            move_direction,
            move_direction_error,
            distance,
            distance_error,
            lines,
            lines_error,
            obstacles,
            obstacles_error,
        };

        debug!("Updated state: {:#?}", new_state);

        new_state.into()
    }
}

#[function_component(App)]
pub fn app() -> Html {
    trace!("[App] Rendering");

    // define styles
    let style = use_style!(
        r"
            width: 100%;
            height: 100%;

            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;

            .error {
                color: red;
            }

            .controls {
                display: flex;
                flex-direction: row;
                justify-content: space-between;

                box-sizing: border-box;
                width: 100%;
                padding: 10px 20px;
                position: fixed;
                bottom: 0;
            }

            .controls>div {
                width: 50%;

                display: flex;
                flex-direction: column;
                justify-content: space-between;
                align-items: center;
            }

            .controls>div>h5 {
                margin: 10px auto;
                text-align: center;
            }
        "
    );

    // define state
    let rover_service = use_mut_ref(|| RoverService::new("http://rover/api"));
    let state = use_reducer(AppState::default);

    // define side effects
    {
        let rover_service = rover_service.clone();
        let state = state.clone();
        let move_direction = state.move_direction;

        use_effect_with(move_direction, move |_| {
            trace!("[App] Scheduling move direction update.");

            let mut speed = state.select_speed();
            let move_type = match state.select_move_type() {
                Some(m_t) => m_t,
                None => {
                    speed = 0;
                    MoveType::Forward
                }
            };

            let _ = rover_service.borrow().r#move(
                move_type,
                speed,
                Callback::from(move |status| match status {
                    Err(e) => {
                        trace!("[App] Rover move direction update failed: {:?}", e);
                        state.dispatch(AppAction::MoveDirectionUpdateError(e, move_direction));
                    }
                    _ => {
                        trace!("[App] Rover move direction update succeeded.");
                    }
                }),
            );
        })
    }

    // define callbacks
    let on_sensor_direction_change = {
        let state = state.clone();

        move |dir| state.dispatch(AppAction::SensorDirectionUpdate(dir))
    };

    let on_move_direction_change = {
        let state = state.clone();

        move |dir| state.dispatch(AppAction::MoveDirectionUpdate(dir))
    };

    let mut extra_messages: Vec<String> = vec![];
    if let Some(ref distance_err) = *state.distance_error {
        extra_messages.push(format!("Distance/{}", distance_err));
    }
    if let Some(ref obstactles_err) = *state.obstacles_error {
        extra_messages.push(format!("Obstacles/{}", obstactles_err));
    }
    if let Some(ref look_err) = *state.sensor_direction_error {
        extra_messages.push(format!("Sensors/{}", look_err))
    }

    html! {
        <div class={style}>
            <SensorsData
                left_obstacle={state.obstacles.get(0).unwrap_or(&false)}
                right_obstacle={state.obstacles.get(1).unwrap_or(&false)}
                distance={state.distance}
                messages={extra_messages} />
            <div class="controls">
                <div>
                    <h5>{"Sensor Direction"}</h5>
                    <p>
                        {"Sensor direction "}<b>{"[ "}{state.sensor_direction.0}{" ; "}{state.sensor_direction.1}{" ]"}</b>
                    </p>
                    <DirectionControl
                        controller_id="sensor"
                        control_mode={DirectionControlMode::Multidirectional}
                        module_mode={DirectionModuleMode::Cumulative}
                        on_direction_change={on_sensor_direction_change}
                        size={50} />
                </div>
                <div>
                    <h5>{"Move Control"}</h5>
                    <p>
                        {"Move direction "}<b>{state.move_type_repr()}</b>{" Speed "}<b>{state.select_speed()}</b>
                    </p>
                    <DirectionControl
                        controller_id="platform"
                        on_direction_change={on_move_direction_change}
                        size={50}
                        x_step={8421505} // this increment gives approx 1 unit of speed change
                        y_step={8421505} // per click
                        xinc_title="↻"
                        xdec_title="↺"
                        has_reset={true} />
                </div>
            </div>
        </div>
    }
}

// use css_in_rust::Style;

// use std::collections::hash_map::HashMap;
// use std::time::Duration;
// use yew::services::timeout::TimeoutService;
// use yew::services::Task;
// use yew::{html, Component, ComponentLink, Html, ShouldRender};
// use yewtil::NeqAssign;
//
// use crate::components::direction_control::{
//     DirectionControl, DirectionControlMode, DirectionModuleMode,
// };
// use crate::components::sensors_data::SensorsData;
// use crate::services::rover_service::RoverService;
//

//

//
// pub struct App {
//     link: ComponentLink<Self>,
//     state: State,
//     style: Style,
//
//     rover_service: RoverService,
//     backend_tasks: HashMap<&'static str, Box<dyn Task>>,
// }
//
// impl App {
//
//     fn update_sensor_direction(&mut self, new_direction: (i32, i32)) -> ShouldRender {
//         let old_direction = self.state.sensor_direction.clone();
//
//         if self.state.sensor_direction.neq_assign(new_direction) {
//             self.backend_tasks.remove(LOOK_TASK);
//
//             match self
//                 .rover_service
//                 .look_at(
//                     - self.state.sensor_direction.0 as i16,
//                     - self.state.sensor_direction.1 as i16,
//                     self.link.callback(move |r| match r {
//                         Ok(()) => Msg::SensorDirectionUpdate(new_direction),
//                         Err(e) => Msg::SensorDirectionUpdateError(e, old_direction)
//                     })) {
//                 Ok(task) => {
//                     self.backend_tasks.insert(LOOK_TASK, Box::new(task));
//                 }
//                 Err(e) => {
//                     self.link.send_message(
//                         Msg::SensorDirectionUpdateError(
//                             anyhow!("Failed to request a look: {}", e),
//                             old_direction
//                         )
//                     );
//                 }
//             }
//
//             return true;
//         } else {
//             self.state.sensor_direction_error.take();
//         }
//
//         false
//     }
//
//     fn update_sensor_direction_error(&mut self, e: Error, prev_direction: (i32, i32)) -> ShouldRender {
//         self.backend_tasks.remove(LOOK_TASK);
//         self.state.sensor_direction_error = Some(e);
//         self.state.sensor_direction = prev_direction;
//
//         true
//     }
//
//     fn update_distance(&mut self, new_distance: f32) -> ShouldRender {
//         self.backend_tasks.remove(GET_DISTANCE_TASK);
//         self.state.distance_error.take();
//         self.reschedule_sensors_update();
//         self.state.distance.neq_assign(new_distance)
//     }
//
//     fn update_distance_error(&mut self, e: Error) -> ShouldRender {
//         self.backend_tasks.remove(GET_DISTANCE_TASK);
//         self.state.distance_error = Some(e);
//         self.reschedule_sensors_update();
//         true
//     }
//
//     fn update_lines_state(&mut self, new_lines: Vec<bool>) -> ShouldRender {
//         self.backend_tasks.remove(GET_LINES_TASK);
//         self.state.lines_error.take();
//         self.state.lines = new_lines;
//         self.reschedule_sensors_update();
//         true
//     }
//
//     fn update_lines_error(&mut self, e: Error) -> ShouldRender {
//         self.backend_tasks.remove(GET_LINES_TASK);
//         self.state.lines_error = Some(e);
//         self.reschedule_sensors_update();
//         true
//     }
//
//     fn update_obstacles_state(&mut self, new_obstacles: Vec<bool>) -> ShouldRender {
//         self.backend_tasks.remove(GET_OBSTACLES_TASK);
//         self.state.obstacles_error.take();
//         self.state.obstacles = new_obstacles;
//         self.reschedule_sensors_update();
//         true
//     }
//
//     fn update_obstacles_error(&mut self, e: Error) -> ShouldRender {
//         self.backend_tasks.remove(GET_OBSTACLES_TASK);
//         self.state.obstacles_error = Some(e);
//         self.reschedule_sensors_update();
//         true
//     }
//
//     fn reschedule_sensors_update(&mut self) {
//         if !self.backend_tasks.contains_key(GET_DISTANCE_TASK)
//             && !self.backend_tasks.contains_key(GET_LINES_TASK)
//             && !self.backend_tasks.contains_key(GET_OBSTACLES_TASK) {
//             self.request_sensors_update();
//         }
//     }
//
//     fn request_sensors_update(&mut self) -> ShouldRender {
//         match self
//             .rover_service
//             .get_distance(self.link.callback(|r| match r {
//                 Ok(d) => Msg::DistanceUpdate(d),
//                 Err(e) => Msg::DistanceUpdateError(e),
//             })) {
//             Ok(task) => {
//                 self.backend_tasks.insert(GET_DISTANCE_TASK, Box::new(task));
//             }
//             Err(e) => {
//                 self.link.send_message(Msg::DistanceUpdateError(anyhow!(
//                     "Failed to request distance: {}",
//                     e
//                 )));
//             }
//         };
//
//         match self
//             .rover_service
//             .get_lines(self.link.callback(|r| match r {
//                 Ok(ls) => Msg::LinesUpdate(ls),
//                 Err(e) => Msg::LinesUpdateError(e),
//             })) {
//             Ok(task) => {
//                 self.backend_tasks.insert(GET_LINES_TASK, Box::new(task));
//             }
//             Err(e) => {
//                 self.link.send_message(Msg::LinesUpdateError(anyhow!(
//                     "Failed to request line detections: {}",
//                     e
//                 )));
//             }
//         };
//
//         match self
//             .rover_service
//             .get_obstacles(self.link.callback(|r| match r {
//                 Ok(os) => Msg::ObstaclesUpdate(os),
//                 Err(e) => Msg::ObstaclesUpdateError(e),
//             })) {
//             Ok(task) => {
//                 self.backend_tasks.insert(GET_OBSTACLES_TASK, Box::new(task));
//             }
//             Err(e) => {
//                 self.link.send_message(Msg::ObstaclesUpdateError(anyhow!(
//                     "Failed to request line detections: {}",
//                     e
//                 )));
//             }
//         };
//
//         false
//     }
// }
//
// impl Component for App {
//     type Message = Msg;
//     type Properties = ();
//
//     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let state = State::default();
//         let style = Style::create(
//             "App",
//             r"
//                 width: 100%;
//                 height: 100%;
//
//                 display: flex;
//                 flex-direction: column;
//                 justify-content: center;
//                 align-items: center;
//
//                 .error {
//                     color: red;
//                 }
//
//                 .controls {
//                     display: flex;
//                     flex-direction: row;
//                     justify-content: space-between;
//
//                     box-sizing: border-box;
//                     width: 100%;
//                     padding: 10px 20px;
//                     position: fixed;
//                     bottom: 0;
//                 }
//
//                 .controls>div {
//                     width: 50%;
//
//                     display: flex;
//                     flex-direction: column;
//                     justify-content: space-between;
//                     align-items: center;
//                 }
//
//                 .controls>div>h5 {
//                     margin: 10px auto;
//                     text-align: center;
//                 }
//             ",
//         )
//             .unwrap();
//
//         let rover_service = RoverService::new("http://rover/api");
//
//         let sensor_update_handle = TimeoutService::spawn(
//             Duration::from_secs(1),
//             link.callback(|_| Msg::RequestSensors),
//         );
//
//         let mut web_tasks = HashMap::<&str, Box<dyn Task>>::new();
//         web_tasks.insert(REQUEST_SENSORS_TASK, Box::new(sensor_update_handle));
//
//         trace!("Created.");
//
//         App {
//             link,
//             state,
//             style,
//
//             rover_service,
//             backend_tasks: web_tasks,
//         }
//     }
//
//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         debug!("Processing message: {:#?}", msg);
//
//         let should_render = match msg {
//             Msg::RequestSensors => self.request_sensors_update(),
//             Msg::LinesUpdate(ls) => self.update_lines_state(ls),
//             Msg::LinesUpdateError(e) => self.update_lines_error(e),
//             Msg::ObstaclesUpdate(os) => self.update_obstacles_state(os),
//             Msg::ObstaclesUpdateError(e) => self.update_obstacles_error(e),
//             Msg::DistanceUpdate(d) => self.update_distance(d),
//             Msg::DistanceUpdateError(e) => self.update_distance_error(e),
//             Msg::SensorDirectionUpdate(sd) => self.update_sensor_direction(sd),
//             Msg::SensorDirectionUpdateError(e, sd) => self.update_sensor_direction_error(e, sd),
//             Msg::MoveDirectionUpdate(md) => self.update_move_direction(md),
//         };
//
//         trace!(
//             "{} re-render.",
//             if should_render { "Skipping" } else { "Will" }
//         );
//
//         true
//     }
//
//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         false
//     }
//
//     fn view(&self) -> Html {
//         trace!("Rendering.");
//

//
//         return html! {
//             <div class=self.style.clone()>
//                 <SensorsData
//                     left_obstacle={self.state.obstacles.get(0).unwrap_or(&false)}
//                     right_obstacle={self.state.obstacles.get(1).unwrap_or(&false)}
//                     distance={self.state.distance}
//                     messages={extra_messages} />
//                 <div class="controls">
//                     <div>
//                         <h5>{"Sensor Direction"}</h5>
//                         {self.current_sensor_direction()}
//                         <DirectionControl
//                             controller_id="sensor"
//                             control_mode={DirectionControlMode::Multidirectional}
//                             module_mode={DirectionModuleMode::Cumulative}
//                             on_direction_change=self.link.callback(|dir| Msg::SensorDirectionUpdate(dir))
//                             size={50} />
//                     </div>
//                     <div>
//                         <h5>{"Move Control"}</h5>
//                         {self.current_move_direction()}
//                         <DirectionControl
//                             controller_id="platform"
//                             on_direction_change=self.link.callback(|dir| Msg::MoveDirectionUpdate(dir))
//                             size={50}
//                             x_step={10}
//                             y_step={10}
//                             xinc_title="↻"
//                             xdec_title="↺"
//                             has_reset={true} />
//                     </div>
//                 </div>
//             </div>
//         };
//     }
// }
