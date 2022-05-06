use near_sdk::json_types::U128;

use crate::{contract::MemeInteraction, models::{husy::*, meme::{MemeTokenId, MemeTokenView}}};

impl MemeInteraction for HusyContract {
    fn like_meme(&mut self, token_id: MemeTokenId, likes: u64) {
        todo!()
    }

    fn get_memes(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
        category: Option<String>,
        main_page_only: bool,
    ) -> Vec<MemeTokenView> {
        
        vec![]
    }
}