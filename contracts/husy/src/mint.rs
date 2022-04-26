use near_sdk::{near_bindgen, AccountId};

use crate::{
    contract_interface::MintNFT,
    models::{husy::*, meme::MemeToken, meme::MemeTokenId, meme_metadata::MemeTokenMetadata},
    utils::payment::with_storage_payment,
};

#[near_bindgen]
impl MintNFT for HusyContract {
    #[payable]
    fn nft_mint(
        &mut self,
        token_id: MemeTokenId,
        token_metadata: MemeTokenMetadata,
        receiver_id: AccountId,
    ) {
        with_storage_payment(|| {
            let meme = MemeToken {
                owner_id: receiver_id,
            };
            assert!(
                self.memes_by_id.insert(&token_id, &meme).is_none(),
                "Meme already exists"
            );
            self.meme_metadata_by_id.insert(&token_id, &token_metadata);
            self.add_meme_to_owner(&meme.owner_id, &token_id)
        });
    }
}
