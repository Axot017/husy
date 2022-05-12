use async_trait::async_trait;

use crate::domain::{config::app_config::AppConfig, errors::failure::Failure};

#[async_trait(?Send)]
pub trait WalletRepository {
    async fn initialize(&self, app_config: Box<AppConfig>) -> Result<(), Failure>;
}
