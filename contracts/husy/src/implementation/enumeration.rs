use near_sdk::{json_types::U128, near_bindgen, AccountId};

use crate::{
    interface::NFTEnumeration,
    models::{husy::*, meme::MemeTokenView},
};

#[near_bindgen]
impl NFTEnumeration for HusyContract {
    fn nft_total_supply(&self) -> U128 {
        U128(self.meme_metadata_by_id.len().into())
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>)  -> Vec<MemeTokenView> {
        return vec![];
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
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::interface::ContractInit;
    use crate::models::meme::MemeToken;
    use crate::models::meme_metadata::MemeTokenMetadata;

    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
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
        let context = get_context("aaa.testnet".to_string(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let meme_token = MemeToken {
            owner_id: "aaa.testnet".to_string(),
        };
        let meme_token_metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let ids = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
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
        let context = get_context("aaa.testnet".to_string(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("aaa.testnet".to_string());

        let memes = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
        for meme in &memes {
            contract.add_meme_to_owner(&"user.testnet".to_string(), meme)
        }

        let result = contract.nft_supply_for_owner("user.testnet".to_string());

        assert_eq!(result, U128(memes.len() as u128))
    }
}
