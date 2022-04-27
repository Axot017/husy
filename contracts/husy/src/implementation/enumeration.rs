use near_sdk::{json_types::U128, near_bindgen};

use crate::{
    interface::{NFTEnumeration, NFTTokenCore},
    models::{husy::*, meme::MemeTokenView},
};

#[near_bindgen]
impl NFTEnumeration for HusyContract {
    fn nft_total_supply(&self) {}

    fn nft_tokens(&self, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) {}

    fn nft_supply_for_owner(&self, account_id: near_sdk::AccountId) {}

    fn nft_tokens_for_owner(
        &self,
        account_id: near_sdk::AccountId,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u64>,
    ) -> Vec<MemeTokenView> {
        vec![]
    }
}
