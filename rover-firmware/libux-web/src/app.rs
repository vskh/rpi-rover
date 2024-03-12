use std::rc::Rc;

use anyhow::Error;
use log::{debug, error, trace, warn};
use stylist::yew::use_style;
use web_time::SystemTime;
use yew::prelude::*;

use libapi_http::api::MoveType;

use crate::components::direction_control::{
    DirectionControl, DirectionControlMode, DirectionModuleMode,
};
use crate::components::sensors_data::SensorsData;
use crate::services::rover_service::RoverService;

#[derive(Debug)]
pub enum AppAction {
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

#[derive(Debug)]
pub struct AppState {
    pub sensor_direction: (i32, i32),
    pub sensor_direction_error: Rc<Option<Error>>,
    pub move_direction: (i32, i32),
    pub move_direction_error: Rc<Option<Error>>,
    pub distance: f32,
    pub distance_error: Rc<Option<Error>>,
    pub distance_timestamp: SystemTime,
    pub lines: Rc<Vec<bool>>,
    pub lines_error: Rc<Option<Error>>,
    pub lines_timestamp: SystemTime,
    pub obstacles: Rc<Vec<bool>>,
    pub obstacles_error: Rc<Option<Error>>,
    pub obstacles_timestamp: SystemTime
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

impl Default for AppState {
    fn default() -> Self {
        Self {
            sensor_direction: Default::default(),
            sensor_direction_error: Default::default(),
            move_direction: Default::default(),
            move_direction_error: Default::default(),
            distance: Default::default(),
            distance_error: Default::default(),
            distance_timestamp: SystemTime::UNIX_EPOCH,
            lines: Default::default(),
            lines_error: Default::default(),
            lines_timestamp: SystemTime::UNIX_EPOCH,
            obstacles: Default::default(),
            obstacles_error: Default::default(),
            obstacles_timestamp: SystemTime::UNIX_EPOCH
        }
    }
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        trace!("Processing dispatched action: {:?}", action);
        
        let mut sensor_direction = self.sensor_direction.clone();
        let mut sensor_direction_error = self.sensor_direction_error.clone();
        let mut move_direction = self.move_direction.clone();
        let mut move_direction_error = self.move_direction_error.clone();
        let mut distance = self.distance.clone();
        let mut distance_error = self.distance_error.clone();
        let mut distance_timestamp = self.distance_timestamp;
        let mut lines = self.lines.clone();
        let mut lines_error = self.lines_error.clone();
        let mut lines_timestamp = self.lines_timestamp;
        let mut obstacles = self.obstacles.clone();
        let mut obstacles_error = self.obstacles_error.clone();
        let mut obstacles_timestamp = self.obstacles_timestamp;
        
        match action {
            AppAction::SensorDirectionUpdate(dir) => {
                sensor_direction = dir;
                sensor_direction_error = None.into();
            },
            AppAction::SensorDirectionUpdateError(e, dir) => {
                sensor_direction = dir;
                sensor_direction_error = Some(e).into();
            },
            AppAction::MoveDirectionUpdate(dir) => {
                move_direction = dir;
                move_direction_error = None.into();
            },
            AppAction::MoveDirectionUpdateError(e, dir) => {
                move_direction = dir;
                move_direction_error = Some(e).into();
            },
            AppAction::ObstaclesUpdate(v) => {
                obstacles = v.into();
                obstacles_error = None.into();
                obstacles_timestamp = SystemTime::now();
            }
            AppAction::ObstaclesUpdateError(e) => {
                obstacles_error = Some(e).into();
                obstacles_timestamp = SystemTime::now();
            },
            AppAction::DistanceUpdate(d) => {
                distance = d;
                distance_error = None.into();
                distance_timestamp = SystemTime::now();
            },
            AppAction::DistanceUpdateError(e) => {
                distance_error = Some(e).into();
                distance_timestamp = SystemTime::now();
            },
            AppAction::LinesUpdate(v) => {
                lines = v.into();
                lines_error = None.into();
                lines_timestamp = SystemTime::now();
            },
            AppAction::LinesUpdateError(e) => {
                lines_error = Some(e).into();
                lines_timestamp = SystemTime::now();
            }
        };

        let new_state = Self {
            sensor_direction,
            sensor_direction_error,
            move_direction,
            move_direction_error,
            distance,
            distance_error,
            distance_timestamp,
            lines,
            lines_error,
            lines_timestamp,
            obstacles,
            obstacles_error,
            obstacles_timestamp
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
    { // sensor direction
        let rover_service = rover_service.clone();
        let state = state.clone();
        let sensor_direction = state.sensor_direction;

        use_effect_with(sensor_direction, move |_| {
            trace!("[App] Scheduling sensor direction update.");

            match rover_service.borrow().look_at(
                - sensor_direction.0 as i16,
                - sensor_direction.1 as i16,
                Callback::from(move |status| match status {
                    Err(e) => {
                        trace!("[App] Rover look direction update failed: {:?}", e);
                        state.dispatch(AppAction::SensorDirectionUpdateError(e, sensor_direction));
                    }
                    _ => {
                        trace!("[App] Rover look direction update succeeded.");
                    }
                }),
            ) {
                Ok(_) => trace!("[App] Sensor direction update scheduled."),
                Err(e) => error!("[App] Sensor direction update scheduling failed: {:?}", e)
            };
        })
    }
    { // move direction
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

            match rover_service.borrow().r#move(
                move_type,
                speed,
                Callback::from(move |status| match status {
                    Err(e) => {
                        warn!("[App] Rover move direction update failed: {:?}", e);
                        state.dispatch(AppAction::MoveDirectionUpdateError(e, move_direction));
                    }
                    _ => {
                        trace!("[App] Rover move direction update succeeded.");
                    }
                }),
            ) {
                Ok(_) => trace!("[App] Move direction update scheduled."),
                Err(e) => error!("[App] Move direction update scheduling failed: {:?}", e)
            };
        })
    }
    { // distance sensor
        let rover_service = rover_service.clone();
        let state = state.clone();
        let distance_timestamp = state.distance_timestamp;

        use_effect_with(distance_timestamp, move |_| {
            trace!("[App] Scheduling distance sensor query.");

            match rover_service.borrow().get_distance(Callback::from(move |status| match status {
                Err(e) => {
                    warn!("[App] Rover distance sensor query failed: {:?}", e);
                    state.dispatch(AppAction::DistanceUpdateError(e));
                }
                Ok(result) => {
                    trace!("[App] Rover distance sensor query succeeded.");
                    state.dispatch(AppAction::DistanceUpdate(result));
                }
            })) {
                Ok(_) => trace!("[App] Rover distance sensor query scheduled."),
                Err(e) => error!("[App] Distance sensor query scheduling failed: {:?}", e)
            };
        });
    }
    { // lines sensor
        let rover_service = rover_service.clone();
        let state = state.clone();
        let lines_timestamp = state.lines_timestamp;

        use_effect_with(lines_timestamp, move |_| {
            trace!("[App] Scheduling line sensors query.");

            match rover_service.borrow().get_lines(Callback::from(move |status| match status {
                Err(e) => {
                    warn!("[App] Rover line sensors query failed: {:?}", e);
                    state.dispatch(AppAction::LinesUpdateError(e));
                }
                Ok(result) => {
                    trace!("[App] Rover line sensors query succeeded.");
                    state.dispatch(AppAction::LinesUpdate(result));
                }
            })) {
                Ok(_) => trace!("[App] Rover line sensors query scheduled."),
                Err(e) => error!("[App] Line sensors query scheduling failed: {:?}", e)
            };
        });
    }
    { // obstacles sensor
        let rover_service = rover_service.clone();
        let state = state.clone();
        let obstacles_timestamp = state.obstacles_timestamp;

        use_effect_with(obstacles_timestamp, move |_| {
            trace!("[App] Scheduling obstacle sensors query.");

            match rover_service.borrow().get_obstacles(Callback::from(move |status| match status {
                Err(e) => {
                    warn!("[App] Rover obstacle sensors query failed: {:?}", e);
                    state.dispatch(AppAction::ObstaclesUpdateError(e));
                }
                Ok(result) => {
                    trace!("[App] Rover obstacle sensors query succeeded.");
                    state.dispatch(AppAction::ObstaclesUpdate(result));
                }
            })) {
                Ok(_) => trace!("[App] Rover obstacle sensors query scheduled."),
                Err(e) => error!("[App] Obstacle sensors query scheduling failed: {:?}", e)
            };
        });
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
    if let Some(ref lines_err) = *state.lines_error {
        extra_messages.push(format!("Lines/{}", lines_err));
    }
    if let Some(ref obstactles_err) = *state.obstacles_error {
        extra_messages.push(format!("Obstacles/{}", obstactles_err));
    }
    if let Some(ref look_err) = *state.sensor_direction_error {
        extra_messages.push(format!("Look/{}", look_err))
    }
    if let Some(ref move_err) = *state.move_direction_error {
        extra_messages.push(format!("Move/{}", move_err))
    }

    html! {
        <div class={style}>
            <SensorsData
                left_obstacle={state.obstacles.get(0).unwrap_or(&false)}
                right_obstacle={state.obstacles.get(1).unwrap_or(&false)}
                left_line={state.lines.get(0).unwrap_or(&false)}
                right_line={state.lines.get(1).unwrap_or(&false)}
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
