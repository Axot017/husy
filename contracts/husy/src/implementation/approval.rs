use near_sdk::near_bindgen;

use crate::{interface::NFTApproval, models::husy::*};

#[near_bindgen]
impl NFTApproval for HusyContract {
    fn nft_approve(
        &mut self,
        token_id: crate::models::meme::MemeTokenId,
        account_id: near_sdk::AccountId,
        msg: Option<String>,
    ) {
    }

    fn nft_is_approved(
        &self,
        token_id: crate::models::meme::MemeTokenId,
        approved_account_id: near_sdk::AccountId,
        approval_id: Option<u64>,
    ) {
    }

    fn nft_revoke(
        &mut self,
        token_id: crate::models::meme::MemeTokenId,
        account_id: near_sdk::AccountId,
    ) {
    }

    fn nft_revoke_all(&mut self, token_id: crate::models::meme::MemeTokenId) {}
}
