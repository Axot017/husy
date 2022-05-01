use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PromiseOrValue};

use crate::{
    contract::NFTTokenCore,
    ext_contracts::{ext_nft_reciever, ext_self_resolver},
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
};

const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;
const NO_DEPOSIT: Balance = 0;

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
        let sender_id = env::predecessor_account_id();
        self.nft_meme_transfer(sender_id, receiver_id, token_id, approval_id, memo);
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let sender_id = env::predecessor_account_id();
        let token = self.nft_meme_transfer(
            sender_id.clone(),
            receiver_id.clone(),
            token_id.clone(),
            approval_id,
            memo,
        );
        let owner_id = token.owner_id;

        ext_nft_reciever::nft_on_transfer(
            sender_id,
            owner_id.clone(),
            token_id.clone(),
            msg,
            &receiver_id,
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
        )
        .then(ext_self_resolver::nft_resolve_transfer(
            owner_id,
            receiver_id,
            token_id,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
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
    fn some_nft_token() {
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
        let context = get_context("aaa.testnet".to_string(), 10000000);
        testing_env!(context);
        let contract = HusyContract::new_default("aaa.testnet".to_string());

        let result = contract.nft_token("not_existing_id.testnet".to_string());

        assert_eq!(result, None);
    }
}
