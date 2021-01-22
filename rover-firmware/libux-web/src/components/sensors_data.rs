use css_in_rust::Style;
use yew::{html, Component, Properties, ComponentLink, Html, ShouldRender};
use yewtil::NeqAssign;

#[derive(Properties, PartialEq, Clone)]
pub struct SensorsDataProps {
    #[prop_or_default]
    pub left_obstacle: bool,

    #[prop_or_default]
    pub right_obstacle: bool,

    #[prop_or_default]
    pub distance: f32,

    #[prop_or(vec![])]
    pub messages: Vec<String>
}

pub struct SensorsData {
    link: ComponentLink<Self>,
    props: SensorsDataProps,
    style: Style,
}

impl Component for SensorsData {
    type Message = ();
    type Properties = SensorsDataProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "SensorsData",
            r#"
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
            "#)
            .unwrap();

        SensorsData {
            link,
            props,
            style
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = self.props.neq_assign(props);

        should_render
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                <div class="obstacle">{format!("{}", if self.props.left_obstacle { ">>>" } else { "|" })}</div>
                <div class="main">
                    <div class="distance">{format!("{} mm", self.props.distance)}</div>
                    <div class="error">
                        {
                            for self.props.messages.iter().map(|m| { html! { <div>{m}</div> } })
                        }
                    </div>
                </div>
                <div class="obstacle">{format!("{}", if self.props.right_obstacle { "<<<" } else { "|" })}</div>
            </div>
        }
    }
}


