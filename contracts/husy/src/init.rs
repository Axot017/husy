use near_sdk::borsh::BorshSerialize;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::{near_bindgen, AccountId};

use crate::contract_interface::ContractInit;
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
            metadata: LazyOption::new(
                StorageKey::HusyContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        }
    }

    #[init]
    fn new_default(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            HusyNFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "TestNFT".to_string(),
                symbol: "TEST".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}
