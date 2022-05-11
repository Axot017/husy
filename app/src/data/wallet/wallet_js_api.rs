use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window)]
    pub async fn near_initialize(config: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = window)]
    pub fn is_logged_in() -> bool;

    #[wasm_bindgen(catch, js_namespace = window)]
    pub fn request_sign_in() -> Result<(), JsValue>;
}
