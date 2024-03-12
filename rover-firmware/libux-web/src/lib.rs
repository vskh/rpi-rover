use log::Level;
use wasm_bindgen::prelude::*;

use crate::app::App;

mod app;
mod components;
mod services;

#[wasm_bindgen(start)]
pub async fn launch() {
    wasm_logger::init(wasm_logger::Config::new(Level::Trace));

    #[cfg(debug_assertions)]
    {
        info!("Initializing in-browser API mocks in development mode.");
        injectMirage().await; // initialize mock HTTP server within browser in dev mode
    }

    yew::Renderer::<App>::new().render();
}

#[cfg(debug_assertions)]
#[wasm_bindgen(module = "/src/js/test.env.mjs")]
extern "C" {
    async fn injectMirage();
}
