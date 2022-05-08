use near_sdk::AccountId;

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
        owner: Option<AccountId>,
        main_page_only: bool,
    ) -> Vec<MemeTokenView> {
        self.meme_additional_data_by_id
            .iter()
            .filter(|(_key, value)| !main_page_only || value.showed_on_main)
            .filter(|(_key, value)| match (&value.category, &category) {
                (Some(meme_category), Some(category)) => meme_category == category,
                (_, None) => true,
                _ => false,
            })
            .filter_map(|(key, value)| {
                let token = self.memes_by_id.get(&key).unwrap();
                match &owner {
                    Some(owner) if owner != &token.owner_id => None,
                    _ => Some((key, value, token)),
                }
            })
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(self.meme_additional_data_by_id.len()) as usize)
            .map(|(key, value, token)| {
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

#[cfg(test)]
mod test {
    use near_sdk::MockedBlockchain;
    use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId, VMContext};

    use crate::contract::ContractInit;
    use crate::models::meme::MemeToken;
    use crate::models::meme_additional_data::MemeAdditionalData;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id.try_into().unwrap())
            .build()
    }

    fn feed_contract(contract: &mut HusyContract, memes: &Vec<MemeTokenView>) {
        for meme in memes.iter() {
            contract.memes_by_id.insert(
                &meme.token_id,
                &MemeToken {
                    owner_id: meme.owner_id.to_owned(),
                    royalty: meme.royalty.to_owned(),
                    approved_account_ids: meme.approved_account_ids.to_owned(),
                    ..Default::default()
                },
            );
            contract
                .meme_metadata_by_id
                .insert(&meme.token_id, &meme.metadata);
            contract.meme_additional_data_by_id.insert(
                &meme.token_id,
                &MemeAdditionalData {
                    category: meme.category.clone(),
                    likes: meme.likes,
                    showed_on_main: meme.showed_on_main,
                    ..Default::default()
                },
            );
        }
    }

    #[test]
    fn get_memes_with_filters() {
        let owner_id = "owner_id.testnet".to_string();
        let context = get_context(owner_id.clone());
        testing_env!(context);
        let mut contract = HusyContract::new_default(owner_id.clone());

        let category = "some_category".to_string();
        let showed_on_main = true;
        let from_index = 2u128;
        let limit = 2u64;
        let expected_result = vec![
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "expected_token_id_1.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "expected_token_id_2.testnet".to_string(),
                ..Default::default()
            },
        ];
        let full_tokens_list = vec![
            MemeTokenView {
                owner_id: "invalid_owner.testnet".to_string(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "invalid_owner_token_id_1.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some("invalid_category".to_string()),
                showed_on_main,
                token_id: "invalid_category_token_id_1.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main: false,
                token_id: "not_main_token_id_1.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "valid_skipped_token_id_1.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "valid_skipped_token_id_2.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "expected_token_id_1.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: "invalid_owner.testnet".to_string(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "invalid_owner_token_id_2.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some("invalid_category".to_string()),
                showed_on_main,
                token_id: "invalid_category_token_id_2.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main: false,
                token_id: "not_main_token_id_2.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "expected_token_id_2.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: "invalid_owner.testnet".to_string(),
                category: Some(category.clone()),
                showed_on_main,
                token_id: "invalid_owner_token_id_3.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some("invalid_category".to_string()),
                showed_on_main,
                token_id: "invalid_category_token_id_3.testnet".to_string(),
                ..Default::default()
            },
            MemeTokenView {
                owner_id: owner_id.clone(),
                category: Some(category.clone()),
                showed_on_main: false,
                token_id: "not_main_token_id_3.testnet".to_string(),
                ..Default::default()
            },
        ];
        feed_contract(&mut contract, &full_tokens_list);

        let result = contract.get_memes(
            Some(from_index),
            Some(limit),
            Some(category),
            Some(owner_id),
            showed_on_main,
        );

        assert_eq!(result, expected_result);
    }
}