use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
    husy::*,
    meme::{MemeTokenId, MemeTokenView},
    meme_metadata::MemeTokenMetadata,
    storage::StorageKey,
};

use super::hashing::hash_account_id;

impl HusyContract {
    pub(crate) fn add_meme_to_owner(&mut self, owner_id: &AccountId, meme_id: &MemeTokenId) {
        let mut owned_memes = self.memes_per_owner.get(owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::MemePerOwnerInner {
                    account_id_hash: hash_account_id(owner_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        owned_memes.insert(meme_id);

        self.memes_per_owner.insert(owner_id, &owned_memes);
    }

    pub(crate) fn get_meme_view(
        &self,
        id: MemeTokenId,
        metadata: Option<MemeTokenMetadata>,
    ) -> Option<MemeTokenView> {
        let token = self.memes_by_id.get(&id)?;
        let metadata = match metadata {
            Some(metadata) => metadata,
            None => self.meme_metadata_by_id.get(&id)?,
        };
        return Some(MemeTokenView {
            metadata,
            owner_id: token.owner_id,
            token_id: id,
        });
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
    fn get_meme_view_with_metadata() {
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
        contract
            .memes_by_id
            .insert(&"id.testnet".to_string(), &meme_token.clone());
        contract
            .meme_metadata_by_id
            .insert(&"id.testnet".to_string(), &meme_token_metadata.to_owned());

        let result =
            contract.get_meme_view("id.testnet".to_string(), Some(meme_token_metadata.clone()));

        assert_eq!(
            result,
            Some(MemeTokenView {
                metadata: meme_token_metadata,
                owner_id: "aaa.testnet".to_string(),
                token_id: "id.testnet".to_string(),
            })
        )
    }

    #[test]
    fn get_meme_view_of_not_existing_token() {
        let context = get_context("bbb.testnet".to_string(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("bbb.testnet".to_string());

        let meme_token = MemeToken {
            owner_id: "aaa.testnet".to_string(),
        };
        let meme_token_metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        contract
            .memes_by_id
            .insert(&"aa.testnet".to_string(), &meme_token.clone());
        contract
            .meme_metadata_by_id
            .insert(&"aa.testnet".to_string(), &meme_token_metadata.to_owned());

        let result = contract.get_meme_view("wrong_id.testnet".to_string(), None);

        assert_eq!(result, None)
    }

    #[test]
    fn get_meme_view_without_metadata() {
        let context = get_context("ccc.testnet".to_string(), 10000000);
        testing_env!(context);
        let mut contract = HusyContract::new_default("ccc.testnet".to_string());

        let meme_token = MemeToken {
            owner_id: "ccc.testnet".to_string(),
        };
        let meme_token_metadata = MemeTokenMetadata {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            ..Default::default()
        };
        contract
            .memes_by_id
            .insert(&"bb.testnet".to_string(), &meme_token.clone());
        contract
            .meme_metadata_by_id
            .insert(&"bb.testnet".to_string(), &meme_token_metadata.to_owned());

        let result = contract.get_meme_view("bb.testnet".to_string(), None);

        assert_eq!(
            result,
            Some(MemeTokenView {
                metadata: meme_token_metadata,
                owner_id: "ccc.testnet".to_string(),
                token_id: "bb.testnet".to_string(),
            })
        )
    }
}
