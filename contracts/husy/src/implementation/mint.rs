use near_sdk::{near_bindgen, AccountId};

use crate::{
    contract::MintNFT,
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

#[cfg(test)]
mod test {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::{Balance, MockedBlockchain};

    use crate::contract::ContractInit;

    use super::*;

    fn get_context(
        predecessor_account_id: String,
        storage_usage: u64,
        attached: Balance,
    ) -> VMContext {
        VMContext {
            current_account_id: "current.testnet".to_string(),
            signer_account_id: "signer.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: attached,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn success_nft_mint() {
        let attached = 999999999999999999999999999;
        let context = get_context("aaa.testnet".to_string(), 10000000, attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let token_id = "token.testnet".to_string();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_string();

        contract.nft_mint(token_id.clone(), metadata.clone(), receiver_id.clone());

        assert_eq!(
            contract.memes_by_id.get(&token_id),
            Some(MemeToken {
                owner_id: receiver_id.clone(),
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
        let context = get_context("aaa.testnet".to_string(), 10000000, attached);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let token_id = "token.testnet".to_string();
        let metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let receiver_id = "receiver.testnet".to_string();

        contract.nft_mint(token_id.clone(), metadata.clone(), receiver_id.clone());
        contract.nft_mint(token_id.clone(), metadata.clone(), receiver_id.clone());
    }
}
