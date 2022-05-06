use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PromiseOrValue, PromiseResult};
use std::collections::HashMap;

use crate::{
    contract::NFTTokenCore,
    ext_contracts::{ext_nft_reciever, ext_self_resolver},
    models::{
        husy::*,
        meme::{MemeTokenId, MemeTokenView},
    },
    utils::payment::{refund_approved_account_ids, with_refund},
};

const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl NFTTokenCore for HusyContract {
    fn nft_token(&self, token_id: MemeTokenId) -> Option<MemeTokenView> {
        self.get_meme_view(token_id, None)
    }

    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        let sender_id = env::predecessor_account_id();
        with_refund(|| {
            self.nft_meme_transfer(sender_id, receiver_id, token_id, approval_id, memo);
        }, None)
    }

    #[payable]
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
            token.approved_account_ids,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
    }

    #[private]
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool {
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                if !return_token {
                    refund_approved_account_ids(owner_id, &approved_account_ids);
                    return true;
                }
            }
        }

        let mut token = match self.memes_by_id.get(&token_id) {
            Some(token) => token,
            None => {
                refund_approved_account_ids(owner_id, &approved_account_ids);
                return true;
            }
        };

        if token.owner_id != receiver_id {
            refund_approved_account_ids(owner_id, &approved_account_ids);
            return true;
        }

        token.owner_id = owner_id.clone();
        self.memes_by_id.insert(&token_id, &token);
        self.swap_meme_owner(&receiver_id, &owner_id, &token_id);

        refund_approved_account_ids(receiver_id, &token.approved_account_ids);

        false
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
            approved_account_ids: HashMap::from([("approved.testnet".to_string(), 0)]),
            next_approval_id: 1,
            royalty: HashMap::from([("royality.testnet".to_string(), 1000)]),
            ..Default::default()
        };
        let meme_token_metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        let meme_id = "meme.testnet".to_string();

        contract.memes_by_id.insert(&meme_id, &meme_token);
        contract.meme_additional_data_by_id.insert(&meme_id, &Default::default());
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
                approved_account_ids: HashMap::from([("approved.testnet".to_string(), 0)]),
                royalty: HashMap::from([("royality.testnet".to_string(), 1000)]),
                ..Default::default()
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
