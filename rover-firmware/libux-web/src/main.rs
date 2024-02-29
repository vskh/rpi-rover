use log::Level;
use yew::prelude::*;

use crate::app::App;

mod app;
// mod components;
// mod services;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Trace));
    yew::Renderer::<App>::new().render();
}
