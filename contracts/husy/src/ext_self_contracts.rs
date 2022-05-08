use crate::models::meme::MemeTokenId;
use near_sdk::ext_contract;
use std::collections::HashMap;

#[ext_contract(ext_self_resolver)]
pub trait NFTResolver {
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool;
}
