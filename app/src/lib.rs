use crate::app::App;
use wasm_bindgen::prelude::wasm_bindgen;

extern crate stdweb;

mod app;
mod data;
mod domain;
mod presentation;
mod use_cases;

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<App>();
}
