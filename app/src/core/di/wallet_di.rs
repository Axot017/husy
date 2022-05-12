use crate::{
    data::wallet::near_wallet_repository::NearWalletRepository,
    use_cases::wallet::initialize_wallet_use_case::InitializeWalletUseCase,
};

use super::get::GetIt;

impl GetIt for InitializeWalletUseCase<NearWalletRepository> {
    fn get() -> Self {
        InitializeWalletUseCase::new(GetIt::get(), GetIt::get())
    }
}

impl GetIt for NearWalletRepository {
    fn get() -> Self {
        NearWalletRepository::new()
    }
}
