use near_sdk::near_bindgen;

use crate::contract::NFTContractMetadata;
use crate::models::husy::*;
use crate::models::husy_metadata::HusyNFTContractMetadata;

#[near_bindgen]
impl NFTContractMetadata for HusyContract {
    fn nft_metadata(&self) -> HusyNFTContractMetadata {
        self.metadata.get().expect("Failed to get metadata")
    }
}

#[cfg(test)]
mod test {
    use near_sdk::{testing_env, VMContext};

    use crate::contract::ContractInit;
    use near_sdk::MockedBlockchain;

    use super::*;

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
    fn success_nft_metadata() {
        let context = get_context("aaa.testnet".to_string(), 10000000);
        testing_env!(context);

        let metadata = HusyNFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "TestNFT".to_string(),
            symbol: "TEST".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        let contract = HusyContract::new("aaa.testnet".to_string(), metadata.clone());

        let result = contract.nft_metadata();

        assert_eq!(result, metadata)
    }
}
