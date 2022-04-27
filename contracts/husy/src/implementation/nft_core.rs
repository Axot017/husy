use near_sdk::{near_bindgen, AccountId};

use crate::{
    interface::NFTTokenCore,
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
};

#[near_bindgen]
impl NFTTokenCore for HusyContract {
    fn nft_token(&self, token_id: MemeTokenId) -> Option<MemeTokenView> {
        self.memes_by_id.get(&token_id).map(|meme| {
            let metadata = self
                .meme_metadata_by_id
                .get(&token_id)
                .expect("Falied to get metadata");
            MemeTokenView {
                token_id,
                owner_id: meme.owner_id,
                metadata,
            }
        })
    }

    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) {
    }

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: MemeTokenId,
        msg: String,
    ) {
    }

    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
    ) {
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
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
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
    fn some_nft_token() {
        let context = get_context("aaa.testnet".to_string(), 0);
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
        let meme_id = "meme.testnet".to_string();

        contract.memes_by_id.insert(&meme_id, &meme_token);
        contract
            .meme_metadata_by_id
            .insert(&meme_id, &meme_token_metadata);

        let result = contract.nft_token(meme_id.clone());

        assert_eq!(
            result,
            Some(MemeTokenView {
                token_id: meme_id,
                owner_id: "aaa.testnet".to_string(),
                metadata: meme_token_metadata,
            })
        );
    }

    #[test]
    fn none_nft_token() {
        let context = get_context("aaa.testnet".to_string(), 0);
        testing_env!(context);
        let contract = HusyContract::new_default("aaa.testnet".to_string());

        let result = contract.nft_token("not_existing_id.testnet".to_string());

        assert_eq!(result, None)
    }
}
