use near_sdk::ext_contract;

use crate::models::meme::MemeTokenId;

#[ext_contract(ext_self_resolver)]
pub trait NFTResolver {
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
    ) -> bool;
}

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
