use crate::{app::App, data::config::dtos::app_config_dto::AppConfigDto, domain::config::app_config::AppConfig};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

extern crate stdweb;

mod app;
mod core;
mod data;
mod domain;
mod presentation;
mod use_cases;

#[wasm_bindgen]
pub fn run_app(a: &JsValue) {
    let config_dto = a.into_serde::<AppConfigDto>().unwrap();
    AppConfig::from(config_dto).save();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
