use near_sdk::{near_bindgen, AccountId};

use crate::{utils::with_storage_payment, *};

#[near_bindgen]
impl HusyContract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        meme_token_id: MemeTokenId,
        meme_token_metadata: MemeTokenMetadata,
        receiver_id: AccountId,
    ) {
        with_storage_payment(|| {
            let meme = Meme {
                owner_id: receiver_id,
            };
            assert!(
                self.memes_by_id.insert(&meme_token_id, &meme).is_none(),
                "Meme already exists"
            );
            self.meme_metadata_by_id
                .insert(&meme_token_id, &meme_token_metadata);
            self.add_meme_to_owner(&meme.owner_id, &meme_token_id)
        });
    }
}
