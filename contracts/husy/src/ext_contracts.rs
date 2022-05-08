use near_sdk::ext_contract;

use crate::models::meme::MemeTokenId;
use std::collections::HashMap;

#[ext_contract(ext_nft_reciever)]
pub trait NFTReciever {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: MemeTokenId,
        msg: String,
    ) -> Promise;
}

#[ext_contract(ext_nft_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: MemeTokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}
