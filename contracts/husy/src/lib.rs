use metadata::{HusyNFTContractMetadata, Meme, MemeTokenId, MemeTokenMetadata};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
    near_bindgen, AccountId, CryptoHash, PanicOnDefault,
};

mod metadata;
mod mint;
mod utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct HusyContract {
    pub owner_id: AccountId,
    pub memes_per_owner: LookupMap<AccountId, UnorderedSet<MemeTokenId>>,
    pub memes_by_id: LookupMap<MemeTokenId, Meme>,
    pub meme_metadata_by_id: UnorderedMap<MemeTokenId, MemeTokenMetadata>,
    pub metadata: LazyOption<HusyNFTContractMetadata>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    MemesPerOwner,
    MemessById,
    MemeMetadataById,
    HusyContractMetadata,
    MemePerOwnerInner { account_id_hash: CryptoHash },
}

#[near_bindgen]
impl HusyContract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: HusyNFTContractMetadata) -> Self {
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
    pub fn new_default(owner_id: AccountId) -> Self {
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
