use near_sdk::{near_bindgen, AccountId};

use crate::{
    interface::NFTTokenCore,
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
};

#[near_bindgen]
impl NFTTokenCore for HusyContract {
    fn nft_token(&self, token_id: MemeTokenId) -> Option<MemeTokenView> {
        self.memes_by_id.get(&token_id).map(|meme| {
            let metadata = self
                .meme_metadata_by_id
                .get(&token_id)
                .expect("Falied to get metadata");
            MemeTokenView {
                token_id,
                owner_id: meme.owner_id,
                metadata,
            }
        })
    }

    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) {
    }

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: MemeTokenId,
        msg: String,
    ) {
    }

    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
    ) {
    }
}
