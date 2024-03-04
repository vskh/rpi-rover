use log::trace;
use stylist::yew::use_style;
use yew::{
    function_component, html, use_state, AttrValue, Callback, Html, MouseEvent, Properties,
    UseStateHandle,
};

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
    Multidirectional,
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
    Mixed,
}

#[derive(Properties, PartialEq, Clone)]
pub struct DirectionControlProps {
    pub controller_id: AttrValue,

    #[prop_or_default]
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

    #[prop_or(AttrValue::from("↑"))]
    pub yinc_title: AttrValue,

    #[prop_or(AttrValue::from("↓"))]
    pub ydec_title: AttrValue,

    #[prop_or(AttrValue::from("→"))]
    pub xinc_title: AttrValue,

    #[prop_or(AttrValue::from("←"))]
    pub xdec_title: AttrValue,

    #[prop_or(AttrValue::from("■"))]
    pub reset_title: AttrValue,
}

#[function_component(DirectionControl)]
pub fn direction_control(props: &DirectionControlProps) -> Html {
    fn create_control_cb(
        var: UseStateHandle<i32>,
        dvar: UseStateHandle<i32>,
        step: i32,
        increment: bool,
        control_mode: DirectionControlMode,
        module_mode: DirectionModuleMode,
        on_change: impl Fn(i32, i32),
    ) -> impl Fn(MouseEvent) {
        move |_| {
            if control_mode == DirectionControlMode::Unidirectional {
                dvar.set(0);
            }

            match module_mode {
                DirectionModuleMode::Immediate => {
                    if increment {
                        var.set(step);
                    } else {
                        var.set(-step);
                    }
                }
                DirectionModuleMode::Mixed => {
                    if (!increment && *var > 0) || (increment && *var < 0) {
                        var.set(0);
                    } else {
                        var.set(if increment {
                            var.saturating_add(step)
                        } else {
                            var.saturating_sub(step)
                        });
                    }
                }
                DirectionModuleMode::Cumulative => {
                    var.set(if increment {
                        var.saturating_add(step)
                    } else {
                        var.saturating_sub(step)
                    });
                }
            }

            // can't call on_direction_change cb directly here because that would require
            // knowledge of order between controlled variable and dependent variable (e.g. (x, y) or (y, x))
            on_change(*var, *dvar)
        }
    }

    let class = use_style!(
        r"
            width: ${c_h}px;
            height: ${c_w}px;

            display: flex;
            flex-direction: column;

            .row {
                display: flex;
                flex-direction: row;
                justify-content: space-between;
            }

            .row-odd {
                justify-content: space-around;
            }

            button {
                line-height: 0;
                padding: 0;
                margin: 3px;
                height: ${h}px;
                width: ${w}px;
            }
        ",
        c_h = (props.size + 6) * 3,
        c_w = (props.size + 6) * 3,
        h = props.size,
        w = props.size
    );

    let x = use_state(|| 0);
    let y = use_state(|| 0);

    trace!(
        "[DirectionControl(#{})] Rendering: ({}, {})",
        props.controller_id,
        *x,
        *y
    );

    let xdec = {
        let on_direction_change = props.on_direction_change.clone();
        let controller_id = props.controller_id.clone();

        create_control_cb(
            x.clone(),
            y.clone(),
            props.x_step,
            false,
            props.control_mode,
            props.module_mode,
            move |x, y| {
                trace!(
                    "[DirectionControl(#{})] Decrementing X: ({}, {})",
                    controller_id,
                    x,
                    y
                );
                on_direction_change.emit((x, y));
            },
        )
    };
    let xinc = {
        let on_direction_change = props.on_direction_change.clone();
        let controller_id = props.controller_id.clone();

        create_control_cb(
            x.clone(),
            y.clone(),
            props.x_step,
            true,
            props.control_mode,
            props.module_mode,
            move |x, y| {
                trace!(
                    "[DirectionControl(#{})] Incrementing X: ({}, {})",
                    controller_id,
                    x,
                    y
                );
                on_direction_change.emit((x, y));
            },
        )
    };

    let ydec = {
        let on_direction_change = props.on_direction_change.clone();
        let controller_id = props.controller_id.clone();

        create_control_cb(
            y.clone(),
            x.clone(),
            props.x_step,
            false,
            props.control_mode,
            props.module_mode,
            move |y, x| {
                trace!(
                    "[DirectionControl(#{})] Decrementing Y: ({}, {})",
                    controller_id,
                    x,
                    y
                );
                on_direction_change.emit((x, y));
            },
        )
    };
    let yinc = {
        let on_direction_change = props.on_direction_change.clone();
        let controller_id = props.controller_id.clone();

        create_control_cb(
            y.clone(),
            x.clone(),
            props.x_step,
            true,
            props.control_mode,
            props.module_mode,
            move |x, y| {
                trace!(
                    "[DirectionControl(#{})] Incrementing Y: ({}, {})",
                    controller_id,
                    x,
                    y
                );
                on_direction_change.emit((y, x));
            },
        )
    };

    let reset = {
        let x = x.clone();
        let y = y.clone();
        let on_direction_change = props.on_direction_change.clone();
        let controller_id = props.controller_id.clone();

        move |_| {
            trace!(
                "[DirectionControl(#{})] Resetting: ({}, {})",
                controller_id,
                *x,
                *y
            );

            x.set(0);
            y.set(0);

            on_direction_change.emit((0, 0));
        }
    };

    html! {
            <div class={class}>
                <div class="row row-odd">
                    <button onclick={yinc}>{&props.yinc_title}</button>
                </div>
                <div class="row">
                    <button onclick={xdec}>{&props.xdec_title}</button>
                    {
                        if props.has_reset {
                            html!{
                                <button onclick={reset}>{&props.reset_title}</button>
                            }
                        } else {
                            html!{}
                        }
                    }
                    <button onclick={xinc}>{&props.xinc_title}</button>
                </div>
                <div class="row row-odd">
                    <button onclick={ydec}>{&props.ydec_title}</button>
                </div>
            </div>
        }
}

