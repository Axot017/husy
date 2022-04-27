use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

use super::meme_metadata::MemeTokenMetadata;

pub type MemeTokenId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MemeToken {
    pub owner_id: AccountId,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MemeTokenView {
    pub token_id: MemeTokenId,
    pub owner_id: AccountId,
    pub metadata: MemeTokenMetadata,
}
