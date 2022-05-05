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
                royalty: royalties.unwrap_or(HashMap::new()),
                ..Default::default()
            };
            assert!(
                self.memes_by_id.insert(&token_id, &meme).is_none(),
                "Meme already exists"
            );

            self.meme_metadata_by_id.insert(&token_id, &token_metadata);
            self.add_meme_to_owner(&meme.owner_id, &token_id)
        }, None);
    }
}

#[cfg(test)]
mod test {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::{Balance, MockedBlockchain};

    use crate::contract::ContractInit;

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
        let context = get_context("aaa.testnet".to_string(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let token_id = "token.testnet".to_string();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_string();

        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            Some(HashMap::from([
                ("account1.testnet".to_string(), 9_999),
                ("account2.testnet".to_string(), 20),
            ])),
        );
    }

    #[test]
    #[should_panic]
    fn nft_mint_panic_to_much_royalities() {
        let attached = 0;
        let context = get_context("aaa.testnet".to_string(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let token_id = "token.testnet".to_string();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_string();

        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            Some(HashMap::from([
                ("account1.testnet".to_string(), 10),
                ("account2.testnet".to_string(), 20),
                ("account3.testnet".to_string(), 20),
                ("account4.testnet".to_string(), 20),
                ("account5.testnet".to_string(), 20),
                ("account6.testnet".to_string(), 20),
            ])),
        );
    }

    #[test]
    fn success_nft_mint() {
        let attached = 999999999999999999999999999;
        let context = get_context("aaa.testnet".to_string(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let token_id = "token.testnet".to_string();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_string();

        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            Some(HashMap::from([
                ("account1.testnet".to_string(), 10),
                ("account2.testnet".to_string(), 20),
            ])),
        );

        assert_eq!(
            contract.memes_by_id.get(&token_id),
            Some(MemeToken {
                owner_id: receiver_id.clone(),
                royalty: HashMap::from([
                    ("account1.testnet".to_string(), 10),
                    ("account2.testnet".to_string(), 20),
                ]),
                ..Default::default()
            })
        );
        assert_eq!(
            contract.meme_metadata_by_id.get(&token_id),
            Some(metadata.clone())
        );
        assert_eq!(
            contract
                .memes_per_owner
                .get(&receiver_id.clone())
                .unwrap()
                .as_vector()
                .get(0)
                .unwrap(),
            token_id.clone()
        )
    }

    #[test]
    #[should_panic]
    fn token_already_exist() {
        let attached = 9999999999999999999999999999;
        let context = get_context("aaa.testnet".to_string(), attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let token_id = "token.testnet".to_string();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_string();

        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            None,
        );
        contract.nft_mint(
            token_id.clone(),
            metadata.clone(),
            receiver_id.clone(),
            None,
        );
    }
}
