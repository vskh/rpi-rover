use log::{debug, trace};
use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::direction_control::{DirectionControl, DirectionControlMode, DirectionModuleMode};

pub enum Msg {
    UpdateSensorDirection((i32, i32)),
    UpdateMoveDirection((i32, i32))
}

pub struct App {
    link: ComponentLink<Self>,
    style: Style,

    sensor_direction: (i32, i32),
    move_direction: (i32, i32)
}

impl App {
    fn current_distance(&self) -> Html {
        html! {
            <p>
                {"Distance: "}{0.0}
            </p>
        }
    }

    fn current_sensor_direction(&self) -> Html {
        html! {
            <p>
                {"Sensor direction "}<b>{"[ "}{self.sensor_direction.0}{" ; "}{self.sensor_direction.1}{" ]"}</b>
            </p>
        }
    }

    fn current_move_direction(&self) -> Html {
        let mut direction = "■";

        if self.move_direction.1 > 0 {
            direction = "↑";
        } else if self.move_direction.1 < 0 {
            direction = "↓";
        } else if self.move_direction.0 > 0 {
            direction = "↻";
        } else if self.move_direction.0 < 0 {
            direction = "↺";
        }

        let mut speed = 0;

        if self.move_direction.0 != 0 {
            speed = self.move_direction.0;
        } else if self.move_direction.1 != 0 {
            speed = self.move_direction.1;
        }

        return html! {
            <p>
                {"Move direction "}<b>{direction}</b>{" Speed "}<b>{speed}</b>
            </p>
        }
    }

    fn update_sensor_direction(&mut self, new_direction: (i32, i32)) {
        self.sensor_direction = new_direction;
    }

    fn update_move_direction(&mut self, new_direction: (i32, i32)) {
        self.move_direction = new_direction;
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "App",
            r"
                width: 100%;
                height: 100%;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

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
            ")
            .unwrap();

        App {
            link,
            style,
            sensor_direction: (0, 0),
            move_direction: (0, 0)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSensorDirection(sd) => self.update_sensor_direction(sd),
            Msg::UpdateMoveDirection(md) => self.update_move_direction(md)
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        trace!("Re-rendering.");

        return html! {
            <div class=self.style.clone()>
                {self.current_distance()}
                <div class="controls">
                    <div>
                        <h5>{"Sensor Direction"}</h5>
                        {self.current_sensor_direction()}
                        <DirectionControl
                            controller_id="sensor"
                            control_mode={DirectionControlMode::Multidirectional}
                            module_mode={DirectionModuleMode::Cumulative}
                            on_direction_change=self.link.callback(|dir| Msg::UpdateSensorDirection(dir))
                            size={50} />
                    </div>
                    <div>
                        <h5>{"Move Control"}</h5>
                        {self.current_move_direction()}
                        <DirectionControl
                            controller_id="platform"
                            on_direction_change=self.link.callback(|dir| Msg::UpdateMoveDirection(dir))
                            size={50}
                            xinc_title="↻"
                            xdec_title="↺"
                            has_reset={true} />
                    </div>
                </div>
            </div>
        };
    }
}

