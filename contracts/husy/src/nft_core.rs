use crate::{metadata::MemeJson, *};
use near_sdk::near_bindgen;

use crate::HusyContract;

#[near_bindgen]
impl HusyContract {
    pub fn nft_token(&self, token_id: MemeTokenId) -> Option<MemeJson> {
        self.memes_by_id.get(&token_id).map(|meme| {
            let metadata = self
                .meme_metadata_by_id
                .get(&token_id)
                .expect("Falied to get metadata");
            MemeJson {
                token_id,
                owner_id: meme.owner_id,
                metadata,
            }
        })
    }
}
