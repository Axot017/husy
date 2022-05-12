use async_trait::async_trait;
use wasm_bindgen::JsValue;

use crate::{
    data::wallet::{dtos::near_config_dto::NearConfigDto, wallet_js_api},
    domain::{config::app_config::AppConfig, errors::failure::Failure},
    use_cases::wallet::ports::wallet_repository::WalletRepository,
};

pub struct NearWalletRepository {}

impl NearWalletRepository {
    pub fn new() -> Self {
        NearWalletRepository {}
    }
}

#[async_trait(?Send)]
impl WalletRepository for NearWalletRepository {
    async fn initialize(&self, app_config: Box<AppConfig>) -> Result<(), Failure> {
        let near_config = JsValue::from_serde(&NearConfigDto::from(app_config))
            .map_err(|_| Failure::Serialization)?;

        wallet_js_api::near_initialize(near_config)
            .await
            .map_err(|_| Failure::WalletInitializetion)
    }
}
