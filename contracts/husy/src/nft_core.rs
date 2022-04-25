use near_sdk::near_bindgen;

use crate::{
    contract_interface::NFTToken,
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
};

#[near_bindgen]
impl NFTToken for HusyContract {
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
}
