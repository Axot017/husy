use std::collections::HashMap;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

use super::meme_metadata::MemeTokenMetadata;

pub type MemeTokenId = String;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Clone, Default, Debug)]
pub struct MemeToken {
    pub owner_id: AccountId,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub next_approval_id: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MemeTokenView {
    pub token_id: MemeTokenId,
    pub owner_id: AccountId,
    pub metadata: MemeTokenMetadata,
    pub approved_account_ids: HashMap<AccountId, u64>,
}
