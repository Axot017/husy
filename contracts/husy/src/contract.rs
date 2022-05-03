use std::collections::HashMap;

use near_sdk::{json_types::U128, AccountId, PromiseOrValue};

use crate::models::{
    husy_metadata::HusyNFTContractMetadata,
    meme::{MemeTokenId, MemeTokenView},
    meme_metadata::MemeTokenMetadata,
    payout::Payout,
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
    ) -> PromiseOrValue<bool>;

    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool;
}

pub trait NFTEnumeration {
    fn nft_total_supply(&self) -> U128;

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<MemeTokenView>;

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128;

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
    ) -> bool;

    fn nft_revoke(&mut self, token_id: MemeTokenId, account_id: AccountId);

    fn nft_revoke_all(&mut self, token_id: MemeTokenId);
}

pub trait NFTRoyality {
    fn nft_payout(&self, token_id: MemeTokenId, balance: U128, max_len_payout: u32) -> Payout;

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}
