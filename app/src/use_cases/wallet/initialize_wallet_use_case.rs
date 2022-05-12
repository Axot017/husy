use crate::domain::{config::app_config::AppConfig, errors::failure::Failure};

use super::ports::wallet_repository::WalletRepository;

pub struct InitializeWalletUseCase<T> {
    wallet_repository: T,
    config: Box<AppConfig>,
}

impl<T> InitializeWalletUseCase<T>
where
    T: WalletRepository,
{
    pub fn new(wallet_repository: T, config: Box<AppConfig>) -> Self {
        InitializeWalletUseCase {
            wallet_repository,
            config,
        }
    }

    pub async fn call(&self) -> Result<(), Failure> {
        self.wallet_repository
            .initialize(self.config.to_owned())
            .await
    }
}
