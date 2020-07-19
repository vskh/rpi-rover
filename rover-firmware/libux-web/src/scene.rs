use log::debug;
use css_in_rust::Style;
use yew::{html, Component, Properties, ComponentLink, Html, ShouldRender};

pub struct Scene {
    link: ComponentLink<Self>,
    style: Style,
}

impl Component for Scene {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Scene",
            r#"
                width: 100%;
                height: 100%;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            "#)
            .unwrap();

        Scene {
            link,
            style,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                {"LOL"}
            </div>
        }
    }
}