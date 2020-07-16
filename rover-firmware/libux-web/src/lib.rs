#![recursion_limit = "256"]

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use yew::start_app;
use crate::app::App;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

mod app;

#[wasm_bindgen(start)]
pub fn launch() {
    start_app::<App>();
}
