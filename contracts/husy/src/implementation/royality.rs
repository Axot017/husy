use near_sdk::{json_types::U128, AccountId};

use crate::{
    contract::NFTRoyality,
    models::{husy::*, meme::MemeTokenId, payout::Payout},
};

impl NFTRoyality for HusyContract {
    fn nft_payout(&self, token_id: MemeTokenId, balance: U128, max_len_payout: u32) -> Payout {
        todo!()
    }

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        todo!()
    }
}
