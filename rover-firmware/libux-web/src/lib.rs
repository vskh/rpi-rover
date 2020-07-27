#![recursion_limit = "256"]

use log::Level;
use wasm_bindgen::prelude::*;
use yew::start_app;

use crate::app::App;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod services;

#[wasm_bindgen(start)]
pub fn launch() {
    wasm_logger::init(wasm_logger::Config::new(Level::Trace));
    start_app::<App>();
}
