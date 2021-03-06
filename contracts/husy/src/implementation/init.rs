use near_sdk::borsh::BorshSerialize;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::{near_bindgen, AccountId};

use crate::contract::ContractInit;
use crate::models::global_likes_data::GlobalLikesData;
use crate::models::storage::StorageKey;
use crate::models::{husy::*, husy_metadata::HusyNFTContractMetadata};

#[near_bindgen]
impl ContractInit for HusyContract {
    #[init]
    fn new(owner_id: AccountId, metadata: HusyNFTContractMetadata) -> Self {
        Self {
            owner_id,
            memes_per_owner: LookupMap::new(StorageKey::MemesPerOwner.try_to_vec().unwrap()),
            memes_by_id: LookupMap::new(StorageKey::MemessById.try_to_vec().unwrap()),
            meme_metadata_by_id: UnorderedMap::new(
                StorageKey::MemeMetadataById.try_to_vec().unwrap(),
            ),
            meme_additional_data_by_id: UnorderedMap::new(
                StorageKey::MemeAdditionalData.try_to_vec().unwrap(),
            ),
            metadata: LazyOption::new(
                StorageKey::HusyContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            global_likes_data: LazyOption::new(
                StorageKey::GlobalLikesData.try_to_vec().unwrap(),
                Some(&GlobalLikesData::new()),
            ),
        }
    }

    #[init]
    fn new_default(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            HusyNFTContractMetadata {
                spec: "nft-1.0.0".to_owned(),
                name: "TestNFT".to_owned(),
                symbol: "TEST".to_owned(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}
