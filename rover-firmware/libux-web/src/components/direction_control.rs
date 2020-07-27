use log::trace;
use css_in_rust::Style;
use yew::{html, Component, Properties, ComponentLink, Html, ShouldRender, Callback};
use yewtil::NeqAssign;

/**
 * Designates how control determines where direction vector is targeted.
 */
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DirectionControlMode {
    /**
     * Only one major direction is supported (4 possible major directions).
     * Once direction changes, the other coordinates of direction vector are reset to zero.
     */
    Unidirectional,

    /**
     * Each coordinate of direction vector is independent and direction is determined by all
     * of them.
     */
    Multidirectional
}

/**
 * Designates how control manages the module of direction vector.
 */
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DirectionModuleMode {
    /**
     * Multiple presses of the button:
     * - in same direction: increase corresponding direction vector coordinate;
     * - in opposite direction: decrease corresponding direction vector coordinate.
     */
    Cumulative,

    /**
     * Presses of the button change direction immediately. Number of presses is not relevant.
     */
    Immediate,

    /**
     * Multiple presses of the button in same direction increase corresponding direction vector
     * coordinate.
     * Single press of the button in the opposite direction resets corresponding direction vector
     * coordinate to 0.
     */
    Mixed
}

pub enum Msg {
    YInc,
    XDec,
    XInc,
    YDec,
    Reset
}

pub struct State {
    /* Direction vector coordinates */
    x: i32,
    y: i32
}

pub struct DirectionControl {
    link: ComponentLink<Self>,
    props: DirectionControlProps,
    state: State,
    style: Style
}

#[derive(Properties, PartialEq, Clone)]
pub struct DirectionControlProps {
    pub controller_id: String,

    pub on_direction_change: Callback<(i32, i32)>,

    #[prop_or(100)]
    pub size: u32,

    #[prop_or(DirectionControlMode::Unidirectional)]
    pub control_mode: DirectionControlMode,

    #[prop_or(DirectionModuleMode::Mixed)]
    pub module_mode: DirectionModuleMode,

    #[prop_or_default]
    pub has_reset: bool,

    #[prop_or(1)]
    pub x_step: i32,

    #[prop_or(1)]
    pub y_step: i32,

    #[prop_or("↑".to_string())]
    pub yinc_title: String,

    #[prop_or("↓".to_string())]
    pub ydec_title: String,

    #[prop_or("→".to_string())]
    pub xinc_title: String,

    #[prop_or("←".to_string())]
    pub xdec_title: String,

    #[prop_or("■".to_string())]
    pub reset_title: String
}

impl DirectionControl {
    fn create_styles(props: &DirectionControlProps) -> Style {
        Style::create(
            "DirectionControl",
            format!(r#"
                width: {c_h}px;
                height: {c_w}px;

                display: flex;
                flex-direction: column;

                .row {{
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                }}

                .row-odd {{
                    justify-content: space-around;
                }}

                button {{
                    line-height: 0;
                    padding: 0;
                    margin: 3px;
                    height: {h}px;
                    width: {w}px;
                }}
            "#,
                    c_h = (props.size + 6) * 3,
                    c_w = (props.size + 6) * 3,
                    h = props.size,
                    w = props.size
            ),
        )
            .unwrap()
    }

    fn update_styles(&mut self) {
        self.style = Self::create_styles(&self.props);
    }
}

impl Component for DirectionControl {
    type Message = Msg;
    type Properties = DirectionControlProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Self::create_styles(&props);
        let state = State { x: 0, y: 0 };
        DirectionControl {
            link,
            style,
            props,
            state
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reset => {
                trace!("[{}] Reset", self.props.controller_id);

                self.state.x = 0;
                self.state.y = 0;
            },
            Msg::XDec => {
                trace!("[{}] XDec", self.props.controller_id);

                if self.props.control_mode == DirectionControlMode::Unidirectional {
                    self.state.y = 0;
                }

                match self.props.module_mode {
                    DirectionModuleMode::Immediate => {
                        self.state.x = - self.props.x_step;
                    },
                    DirectionModuleMode::Mixed => {
                        if self.state.x > 0 {
                            self.state.x = 0;
                        } else {
                            self.state.x = self.state.x.saturating_sub(self.props.x_step);
                        }
                    }
                    DirectionModuleMode::Cumulative => {
                        self.state.x = self.state.x.saturating_sub(self.props.x_step);
                    }
                }
            },
            Msg::XInc => {
                trace!("[{}] XInc", self.props.controller_id);

                if self.props.control_mode == DirectionControlMode::Unidirectional {
                    self.state.y = 0;
                }

                match self.props.module_mode {
                    DirectionModuleMode::Immediate => {
                        self.state.x = self.props.x_step
                    },
                    DirectionModuleMode::Mixed => {
                        if self.state.x < 0 {
                            self.state.x = 0
                        } else {
                            self.state.x = self.state.x.saturating_add(self.props.x_step);
                        }
                    }
                    DirectionModuleMode::Cumulative => {
                        self.state.x = self.state.x.saturating_add(self.props.x_step);
                    }
                }
            },
            Msg::YDec => {
                trace!("[{}] YDec", self.props.controller_id);

                if self.props.control_mode == DirectionControlMode::Unidirectional {
                    self.state.x = 0;
                }

                match self.props.module_mode {
                    DirectionModuleMode::Immediate => {
                        self.state.y = - self.props.y_step;
                    },
                    DirectionModuleMode::Mixed =>
                        if self.state.y > 0 {
                            self.state.y = 0;
                        } else {
                            self.state.y = self.state.y.saturating_sub(self.props.y_step);
                        }
                    DirectionModuleMode::Cumulative => {
                        self.state.y = self.state.y.saturating_sub(self.props.y_step);
                    }
                }
            },
            Msg::YInc => {
                trace!("[{}] YInc", self.props.controller_id);

                if self.props.control_mode == DirectionControlMode::Unidirectional {
                    self.state.x = 0;
                }

                match self.props.module_mode {
                    DirectionModuleMode::Immediate => {
                        self.state.y = - self.props.y_step;
                    },
                    DirectionModuleMode::Mixed =>
                        if self.state.y < 0 {
                            self.state.y = 0;
                        } else {
                            self.state.y = self.state.y.saturating_add(self.props.y_step);
                        }
                    DirectionModuleMode::Cumulative => {
                        self.state.y = self.state.y.saturating_add(self.props.y_step);
                    }
                }
            }
        }

        trace!(
            "[{}] Updated direction vector: ({}, {})",
            self.props.controller_id,
            self.state.x,
            self.state.y
        );

        self.props.on_direction_change.emit((self.state.x, self.state.y));

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = self.props.neq_assign(props);
        if should_render {
            self.update_styles();

            trace!("[{}] Re-rendering.", self.props.controller_id);
        }
        should_render
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                <div class="row row-odd">
                    <button onclick=self.link.callback(|_| Msg::YInc)>{&self.props.yinc_title}</button>
                </div>
                <div class="row">
                    <button onclick=self.link.callback(|_| Msg::XDec)>{&self.props.xdec_title}</button>
                    {
                        if self.props.has_reset {
                            html!{
                                <button onclick=self.link.callback(|_| Msg::Reset)>{&self.props.reset_title}</button>
                            }
                        } else {
                            html!{}
                        }
                    }
                    <button onclick=self.link.callback(|_| Msg::XInc)>{&self.props.xinc_title}</button>
                </div>
                <div class="row row-odd">
                    <button onclick=self.link.callback(|_| Msg::YDec)>{&self.props.ydec_title}</button>
                </div>
            </div>
        }
    }
}