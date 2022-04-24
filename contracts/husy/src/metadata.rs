use crate::*;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::Base64VecU8,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

pub type MemeTokenId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct HusyNFTContractMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MemeTokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub copies: Option<u64>,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub starts_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub extra: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Meme {
    pub owner_id: AccountId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MemeJson {
    pub token_id: MemeTokenId,
    pub owner_id: AccountId,
    pub metadata: MemeTokenMetadata,
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self) -> HusyNFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for HusyContract {
    fn nft_metadata(&self) -> HusyNFTContractMetadata {
        self.metadata.get().expect("Failed to get metadata")
    }
}
