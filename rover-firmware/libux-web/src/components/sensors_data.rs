use stylist::yew::use_style;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct SensorsDataProps {
    #[prop_or_default]
    pub left_obstacle: bool,

    #[prop_or_default]
    pub right_obstacle: bool,

    #[prop_or_default]
    pub distance: f32,

    #[prop_or(vec![])]
    pub messages: Vec<String>,
}

#[function_component(SensorsData)]
pub fn sensors_data(props: &SensorsDataProps) -> Html {
    let style = use_style!(
        r"
            width: 100%;
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;

            .main {
                dispay: flex;
                flex-direction: column;
                justify-content: center;
            }

            .distance {
                flex-grow: 1;
                text-align: center;
            }

            .obstacle {
                min-width: 30px;
                text-align: center;
                flex-grow: 0;
            }
        "
    );

    html! {
        <div class={style}>
            <div class="obstacle">{format!("{}", if props.left_obstacle { ">>>" } else { "|" })}</div>
            <div class="main">
                <div class="distance">{format!("{} mm", props.distance)}</div>
                <div class="error">
                    {
                        for props.messages.iter().map(|m| { html! { <div>{m}</div> } })
                    }
                </div>
            </div>
            <div class="obstacle">{format!("{}", if props.right_obstacle { "<<<" } else { "|" })}</div>
        </div>
    }
}
