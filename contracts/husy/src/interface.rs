use near_sdk::{ext_contract, json_types::U128, AccountId};

use crate::models::{
    husy_metadata::HusyNFTContractMetadata,
    meme::{MemeTokenId, MemeTokenView},
    meme_metadata::MemeTokenMetadata,
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

pub trait NFTTokenCore {
    fn nft_token(&self, token_id: MemeTokenId) -> Option<MemeTokenView>;

    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    );

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: MemeTokenId,
        msg: String,
    );

    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
    );
}

pub trait NFTEnumeration {
    fn nft_total_supply(&self) -> U128;

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>);

    fn nft_supply_for_owner(&self, account_id: AccountId);

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<MemeTokenView>;
}

pub trait NFTApproval {
    fn nft_approve(&mut self, token_id: MemeTokenId, account_id: AccountId, msg: Option<String>);

    fn nft_is_approved(
        &self,
        token_id: MemeTokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    );

    fn nft_revoke(&mut self, token_id: MemeTokenId, account_id: AccountId);

    fn nft_revoke_all(&mut self, token_id: MemeTokenId);
}
