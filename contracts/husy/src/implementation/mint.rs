use near_sdk::{near_bindgen, AccountId};
use std::collections::HashMap;

use crate::{
    contract::MintNFT,
    models::{husy::*, meme::MemeToken, meme::MemeTokenId, meme_metadata::MemeTokenMetadata},
    utils::payment::with_refund,
};

#[near_bindgen]
impl MintNFT for HusyContract {
    #[payable]
    fn nft_mint(
        &mut self,
        token_id: MemeTokenId,
        token_metadata: MemeTokenMetadata,
        receiver_id: AccountId,
        royalties: Option<HashMap<AccountId, u32>>,
    ) {
        with_refund(|| {
            if let Some(royalties) = &royalties {
                assert!(
                    royalties.len() <= 5,
                    "Cannot add more than 5 royalities account"
                );
                let sum: u32 = royalties.values().sum();
                assert!(
                    sum < 10_000,
                    "Sum of royalities cannot be bigger than 10 000"
                );
            }
            let meme = MemeToken {
                owner_id: receiver_id,
                royalty: royalties.unwrap_or_default(),
                ..Default::default()
            };
            assert!(
                self.memes_by_id.insert(&token_id, &meme).is_none(),
                "Meme already exists"
            );

            self.meme_metadata_by_id.insert(&token_id, &token_metadata);
            self.meme_additional_data_by_id
                .insert(&token_id, &Default::default());

            (self.add_meme_to_owner(&meme.owner_id, &token_id), None)
        });
    }
}

#[cfg(test)]
mod test {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::{Balance, MockedBlockchain};

    use crate::contract::ContractInit;
    use crate::models::meme_additional_data::MemeAdditionalData;

    use super::*;

    fn get_context(predecessor_account_id: String, attached: Balance) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id.try_into().unwrap())
            .attached_deposit(attached)
            .build()
    }

    #[test]
    #[should_panic]
    fn nft_mint_panic_royalities_sum_to_big() {
        let attached = 0;
        let context = get_context("aaa.testnet".to_owned(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let token_id = "token.testnet".to_owned();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_owned()),
            description: Some("description".to_owned()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_owned();

        contract.nft_mint(
            token_id,
            metadata,
            receiver_id,
            Some(HashMap::from([
                ("account1.testnet".to_owned(), 9_999),
                ("account2.testnet".to_owned(), 20),
            ])),
        );
    }

    #[test]
    #[should_panic]
    fn nft_mint_panic_to_much_royalities() {
        let attached = 0;
        let context = get_context("aaa.testnet".to_owned(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let token_id = "token.testnet".to_owned();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_owned()),
            description: Some("description".to_owned()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_owned();

        contract.nft_mint(
            token_id,
            metadata,
            receiver_id,
            Some(HashMap::from([
                ("account1.testnet".to_owned(), 10),
                ("account2.testnet".to_owned(), 20),
                ("account3.testnet".to_owned(), 20),
                ("account4.testnet".to_owned(), 20),
                ("account5.testnet".to_owned(), 20),
                ("account6.testnet".to_owned(), 20),
            ])),
        );
    }

    #[test]
    fn success_nft_mint() {
        let attached = 999999999999999999999999999;
        let context = get_context("aaa.testnet".to_owned(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let token_id = "token.testnet".to_owned();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_owned()),
            description: Some("description".to_owned()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_owned();

        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            Some(HashMap::from([
                ("account1.testnet".to_owned(), 10),
                ("account2.testnet".to_owned(), 20),
            ])),
        );

        assert_eq!(
            contract.memes_by_id.get(&token_id),
            Some(MemeToken {
                owner_id: receiver_id.clone(),
                royalty: HashMap::from([
                    ("account1.testnet".to_owned(), 10),
                    ("account2.testnet".to_owned(), 20),
                ]),
                ..Default::default()
            })
        );
        assert_eq!(contract.meme_metadata_by_id.get(&token_id), Some(metadata));
        assert_eq!(
            contract
                .memes_per_owner
                .get(&receiver_id)
                .unwrap()
                .as_vector()
                .get(0)
                .unwrap(),
            token_id
        );
        assert_eq!(
            contract.meme_additional_data_by_id.get(&token_id).unwrap(),
            MemeAdditionalData {
                ..Default::default()
            }
        )
    }

    #[test]
    #[should_panic]
    fn token_already_exist() {
        let attached = 9999999999999999999999999999;
        let context = get_context("aaa.testnet".to_owned(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_owned());

        let token_id = "token.testnet".to_owned();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_owned()),
            description: Some("description".to_owned()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_owned();

        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            None,
        );
        contract.nft_mint(token_id, metadata, receiver_id, None);
    }
}
