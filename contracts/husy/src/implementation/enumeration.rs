use near_sdk::{json_types::U128, near_bindgen, AccountId};

use crate::{
    contract::NFTEnumeration,
    models::{husy::*, meme::MemeTokenView},
};

#[near_bindgen]
impl NFTEnumeration for HusyContract {
    fn nft_total_supply(&self) -> U128 {
        U128(self.meme_metadata_by_id.len().into())
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<MemeTokenView> {
        self.meme_metadata_by_id
            .iter()
            .skip(from_index.unwrap_or(U128(0)).0 as usize)
            .take(limit.unwrap_or(self.meme_metadata_by_id.len()) as usize)
            .filter_map(|(key, value)| self.get_meme_view(key, Some(value)))
            .collect()
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.memes_per_owner
            .get(&account_id)
            .map(|memes| U128(memes.len().into()))
            .unwrap_or(U128(0))
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: near_sdk::AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<MemeTokenView> {
        let memes = match self.memes_per_owner.get(&account_id) {
            Some(memes) => memes,
            None => return vec![],
        };

        memes
            .iter()
            .skip(from_index.unwrap_or(U128(0)).0 as usize)
            .take(limit.unwrap_or(self.meme_metadata_by_id.len()) as usize)
            .filter_map(|id| self.get_meme_view(id, None))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::contract::ContractInit;
    use crate::models::meme::MemeToken;
    use crate::models::meme_metadata::MemeTokenMetadata;

    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "current.testnet".to_owned(),
            signer_account_id: "signer.testnet".to_owned(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_nft_total_supply() {
        let context = get_context("aaa.testnet".to_owned(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let meme_token = MemeToken {
            owner_id: "aaa.testnet".to_owned(),
            ..Default::default()
        };
        let meme_token_metadata = MemeTokenMetadata {
            title: Some("title".to_owned()),
            description: Some("description".to_owned()),
            ..Default::default()
        };
        let ids = vec!["aaa".to_owned(), "bbb".to_owned(), "ccc".to_owned()];
        for id in &ids {
            contract.memes_by_id.insert(id, &meme_token.clone());
            contract
                .meme_metadata_by_id
                .insert(id, &meme_token_metadata.to_owned());
        }

        let result = contract.nft_total_supply();

        assert_eq!(result, U128(ids.len() as u128))
    }

    #[test]
    fn test_nft_supply_for_owner() {
        let context = get_context("aaa.testnet".to_owned(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let memes = vec!["aaa".to_owned(), "bbb".to_owned(), "ccc".to_owned()];
        for meme in &memes {
            contract.add_meme_to_owner(&"user.testnet".to_owned(), meme)
        }

        let result = contract.nft_supply_for_owner("user.testnet".to_owned());

        assert_eq!(result, U128(memes.len() as u128))
    }

    #[test]
    fn full_nft_tokens() {
        let context = get_context("aaa.testnet".to_owned(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let ids = vec![
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
            "d".to_owned(),
        ];
        let memes = vec![
            MemeToken {
                owner_id: "a.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "b.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "c.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "d.testnet".to_owned(),
                ..Default::default()
            },
        ];
        let metadatas = vec![
            MemeTokenMetadata {
                title: Some("titleA".to_owned()),
                description: Some("descriptionA".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("titleB".to_owned()),
                description: Some("descriptionB".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("titleC".to_owned()),
                description: Some("descriptionC".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("titleD".to_owned()),
                description: Some("descriptionD".to_owned()),
                ..Default::default()
            },
        ];

        for i in 0..4 {
            let id = &ids[i];
            let meme = &memes[i];
            let metadata = &metadatas[i];
            contract.meme_metadata_by_id.insert(id, metadata);
            contract.memes_by_id.insert(id, meme);
            contract
                .meme_additional_data_by_id
                .insert(id, &Default::default());
        }

        let result = contract.nft_tokens(None, None);

        assert_eq!(result.len(), ids.len());
        for (index, view) in result.iter().enumerate() {
            assert_eq!(view.metadata, metadatas[index]);
            assert_eq!(view.owner_id, memes[index].owner_id);
            assert_eq!(view.token_id, ids[index]);
        }
    }

    #[test]
    fn part_of_nft_tokens() {
        let context = get_context("aaa.testnet".to_owned(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let ids = vec![
            "1".to_owned(),
            "2".to_owned(),
            "3".to_owned(),
            "4".to_owned(),
        ];
        let memes = vec![
            MemeToken {
                owner_id: "1.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "2.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "3.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "3.testnet".to_owned(),
                ..Default::default()
            },
        ];
        let metadatas = vec![
            MemeTokenMetadata {
                title: Some("title1".to_owned()),
                description: Some("description1".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title2".to_owned()),
                description: Some("description2".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title3".to_owned()),
                description: Some("description3".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title4".to_owned()),
                description: Some("description4".to_owned()),
                ..Default::default()
            },
        ];

        for i in 0..4 {
            let id = &ids[i];
            let meme = &memes[i];
            let metadata = &metadatas[i];
            contract.meme_metadata_by_id.insert(id, metadata);
            contract.memes_by_id.insert(id, meme);
            contract
                .meme_additional_data_by_id
                .insert(id, &Default::default());
        }
        let skipped = 1;
        let limit = 2;

        let result = contract.nft_tokens(Some(U128(skipped)), Some(limit));

        assert_eq!(result.len(), limit as usize);
        for (index, view) in result.iter().enumerate() {
            assert_eq!(view.metadata, metadatas[index + skipped as usize]);
            assert_eq!(view.owner_id, memes[index + skipped as usize].owner_id);
            assert_eq!(view.token_id, ids[index + skipped as usize]);
        }
    }

    #[test]
    fn no_nft_tokens_for_owner() {
        let context = get_context("aaa.testnet".to_owned(), 10000000);
        testing_env!(context);
        let contract = HusyContract::new_default("aaa.testnet".to_owned());

        let result = contract.nft_tokens_for_owner("some_account.testnet".to_owned(), None, None);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn show_nft_tokens_for_owner() {
        let context = get_context("owner.testnet".to_owned(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("owner.testnet".to_owned());

        let ids = vec![
            "memeA".to_owned(),
            "memeB".to_owned(),
            "memeC".to_owned(),
            "memeD".to_owned(),
            "memeE".to_owned(),
        ];
        let memes = vec![
            MemeToken {
                owner_id: "other.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "owner.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "owner.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "owner.testnet".to_owned(),
                ..Default::default()
            },
            MemeToken {
                owner_id: "owner.testnet".to_owned(),
                ..Default::default()
            },
        ];
        let metadatas = vec![
            MemeTokenMetadata {
                title: Some("title0".to_owned()),
                description: Some("description0".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title1".to_owned()),
                description: Some("description1".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title2".to_owned()),
                description: Some("description2".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title3".to_owned()),
                description: Some("description3".to_owned()),
                ..Default::default()
            },
            MemeTokenMetadata {
                title: Some("title4".to_owned()),
                description: Some("description4".to_owned()),
                ..Default::default()
            },
        ];

        for i in 0..4 {
            let id = &ids[i];
            let meme = &memes[i];
            let metadata = &metadatas[i];
            contract.add_meme_to_owner(&meme.owner_id, id);
            contract.meme_metadata_by_id.insert(id, metadata);
            contract.memes_by_id.insert(id, meme);
            contract
                .meme_additional_data_by_id
                .insert(id, &Default::default());
        }
        let skipped = 1;
        let limit = 2;

        let result = contract.nft_tokens_for_owner(
            "owner.testnet".to_owned(),
            Some(U128(skipped)),
            Some(limit),
        );

        assert_eq!(result.len(), limit as usize);
        for (index, view) in result.iter().enumerate() {
            // + 1 bacause first meme is owned by different account
            assert_eq!(view.metadata, metadatas[index + skipped as usize + 1]);
            assert_eq!(view.owner_id, memes[index + skipped as usize + 1].owner_id);
            assert_eq!(view.token_id, ids[index + skipped as usize + 1]);
        }
    }
}
