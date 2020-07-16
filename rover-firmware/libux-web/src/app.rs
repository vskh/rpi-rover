use log::debug;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        debug!("Rendering!");

        return html! {
            <div>{ "Hello, world" }</div>
        }
    }
}

