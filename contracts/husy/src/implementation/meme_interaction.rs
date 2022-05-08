use near_sdk::{env, near_bindgen, AccountId};

use crate::{
    contract::MemeInteraction,
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
    utils::likes_helpers::{count_new_likes_state, try_move_to_main},
};

const YOCTO_NEAR_PER_LIKE: u128 = 50_000_000_000_000_000_000_000;

#[near_bindgen]
impl MemeInteraction for HusyContract {
    fn yocto_near_price_for_like(&self) -> u128 {
        YOCTO_NEAR_PER_LIKE
    }

    #[payable]
    fn like_meme(&mut self, meme_id: MemeTokenId, likes: u64) {
        let attached = env::attached_deposit();
        let needed = YOCTO_NEAR_PER_LIKE * likes as u128;
        assert!(
            attached >= needed,
            "Not enought deposit attached. You need at least: {} yoctoNEAR",
            needed
        );

        let meme = self.memes_by_id.get(&meme_id).expect("Meme not found");
        let predecessor_account_id = env::predecessor_account_id();
        assert_ne!(
            &predecessor_account_id, &meme.owner_id,
            "Cannot like own meme"
        );

        let mut meme_additional_data = self.meme_additional_data_by_id.get(&meme_id).unwrap();
        let mut global_likes_data = self.global_likes_data.get().unwrap();

        count_new_likes_state(&mut meme_additional_data, &mut global_likes_data, likes);
        try_move_to_main(&mut meme_additional_data, &global_likes_data);
        global_likes_data.try_switching_mode();
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

    fn get_context(predecessor_account_id: AccountId, attached: u128) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id.try_into().unwrap())
            .attached_deposit(attached)
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
    #[should_panic(expected = "Cannot like own meme")]
    fn like_meme_liking_own_meme() {
        let owner_id = "owner_id.testnet".to_string();
        let context = get_context(owner_id.clone(), YOCTO_NEAR_PER_LIKE);
        testing_env!(context);
        let mut contract = HusyContract::new_default(owner_id.clone());
        let meme_id = "some_meme_id.testnet".to_string();

        contract.memes_by_id.insert(
            &meme_id,
            &MemeToken {
                owner_id: owner_id.clone(),
                ..Default::default()
            },
        );

        contract.like_meme(meme_id, 1);
    }

    #[test]
    #[should_panic]
    fn like_meme_not_enought_attached_deposit() {
        let owner_id = "owner_id.testnet".to_string();
        let context = get_context(owner_id.clone(), 10000);
        testing_env!(context);
        let mut contract = HusyContract::new_default(owner_id.clone());

        contract.like_meme("some_meme.testnet".to_string(), 50);
    }

    #[test]
    fn yocto_near_price_for_like_test() {
        let owner_id = "owner_id.testnet".to_string();
        let context = get_context(owner_id.clone(), 0);
        testing_env!(context);
        let contract = HusyContract::new_default(owner_id.clone());

        let result = contract.yocto_near_price_for_like();
        assert_eq!(result, YOCTO_NEAR_PER_LIKE);
    }

    #[test]
    fn get_memes_with_filters() {
        let owner_id = "owner_id.testnet".to_string();
        let context = get_context(owner_id.clone(), 0);
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
