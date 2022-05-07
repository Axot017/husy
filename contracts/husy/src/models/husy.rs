use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
    near_bindgen, AccountId, PanicOnDefault,
};

use super::{
    global_likes_data::GlobalLikesData,
    husy_metadata::HusyNFTContractMetadata,
    meme::{MemeToken, MemeTokenId},
    meme_additional_data::MemeAdditionalData,
    meme_metadata::MemeTokenMetadata,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct HusyContract {
    pub owner_id: AccountId,
    pub memes_per_owner: LookupMap<AccountId, UnorderedSet<MemeTokenId>>,
    pub memes_by_id: LookupMap<MemeTokenId, MemeToken>,
    pub meme_metadata_by_id: UnorderedMap<MemeTokenId, MemeTokenMetadata>,
    pub meme_additional_data_by_id: UnorderedMap<MemeTokenId, MemeAdditionalData>,
    pub metadata: LazyOption<HusyNFTContractMetadata>,
    pub likes_data: LazyOption<GlobalLikesData>,
}
