
use crate::{
    contract::MemeInteraction,
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
};

impl MemeInteraction for HusyContract {
    fn like_meme(&mut self, token_id: MemeTokenId, likes: u64) {
        todo!()
    }

    fn get_memes(
        &self,
        from_index: Option<u128>,
        limit: Option<u64>,
        category: Option<String>,
        main_page_only: bool,
    ) -> Vec<MemeTokenView> {
        self.meme_additional_data_by_id
            .iter()
            .filter(|(_key, value)| !main_page_only || value.showed_on_main)
            .filter(|(_key, value)| {
                category
                    .as_ref()
                    .map(|category| match &value.category {
                        Some(meme_category) => meme_category == category,
                        None => true,
                    })
                    .unwrap_or(true)
            })
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(self.meme_additional_data_by_id.len()) as usize)
            .map(|(key, value)| {
                let token = self.memes_by_id.get(&key).unwrap();
                let metadata = self.meme_metadata_by_id.get(&key).unwrap();
                MemeTokenView {
                    token_id: key,
                    owner_id: token.owner_id,
                    approved_account_ids: token.approved_account_ids,
                    metadata,
                    royalty: token.royalty,
                    likes: value.likes,
                    showed_on_main: value.showed_on_main,
                    category: value.category,
                }
            })
            .collect()
    }
}
