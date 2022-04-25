use near_sdk::AccountId;

use crate::models::{
    husy_metadata::HusyNFTContractMetadata, meme::{MemeTokenId, MemeTokenView}, meme_metadata::MemeTokenMetadata,
};

pub trait ContractInit {
    fn new(owner_id: AccountId, metadata: HusyNFTContractMetadata) -> Self;

    fn new_default(owner_id: AccountId) -> Self;
}

pub trait NFTContractMetadata {
    fn nft_metadata(&self) -> HusyNFTContractMetadata;
}

pub trait MintNFT {
    fn nft_mint(
        &mut self,
        token_id: MemeTokenId,
        token_metadata: MemeTokenMetadata,
        receiver_id: AccountId,
    );
}

pub trait NFTToken {
    fn nft_token(&self, token_id: MemeTokenId) -> Option<MemeTokenView>;
}
