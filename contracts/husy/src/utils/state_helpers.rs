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

    pub(crate) fn remove_meme_from_owner(&mut self, owner_id: &AccountId, meme_id: &MemeTokenId) {
        let mut owned_memes = self
            .memes_per_owner
            .get(owner_id)
            .expect(format!("User {} has no memes", owner_id).as_str());

        let result = owned_memes.remove(meme_id);

        if !result {
            panic!("User {} does not own meme {}", owner_id, meme_id)
        }

        if owned_memes.is_empty() {
            self.memes_per_owner.remove(owner_id);
        } else {
            self.memes_per_owner.insert(owner_id, &owned_memes);
        }
    }

    pub(crate) fn swap_meme_owner(
        &mut self,
        owner_id: &AccountId,
        receiver_id: &AccountId,
        meme_id: &MemeTokenId,
    ) {
        self.remove_meme_from_owner(owner_id, meme_id);
        self.add_meme_to_owner(receiver_id, meme_id)
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
    use crate::contract::ContractInit;
    use crate::models::meme::MemeToken;
    use crate::models::meme_metadata::MemeTokenMetadata;

    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id.try_into().unwrap())
            .build()
    }

    #[test]
    fn get_meme_view_with_metadata() {
        let context = get_context("aaa.testnet".to_string());
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
        let context = get_context("bbb.testnet".to_string());
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
        let context = get_context("ccc.testnet".to_string());
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

    #[test]
    fn test_add_meme_to_owner() {
        let context = get_context("ccc.testnet".to_string());
        testing_env!(context);
        let mut contract = HusyContract::new_default("ccc.testnet".to_string());

        let user_1 = "aaa.testnet".to_string();
        let user_2 = "bbb.testnet".to_string();

        contract.add_meme_to_owner(&user_1, &"meme1.testnet".to_string());
        contract.add_meme_to_owner(&user_1, &"meme2.testnet".to_string());
        contract.add_meme_to_owner(&user_2, &"meme3.testnet".to_string());

        assert_eq!(contract.memes_per_owner.get(&user_1).unwrap().len(), 2);
        assert_eq!(contract.memes_per_owner.get(&user_2).unwrap().len(), 1);
    }

    #[test]
    fn remove_meme_from_owner_success_one_meme() {
        let owner_id = "meme_owner.testnet".to_string();
        let context = get_context(owner_id.clone());
        testing_env!(context);
        let mut contract = HusyContract::new_default("contract_owner.testnet".to_string());

        let test_meme_id = "test_meme.testnet".to_string();
        contract.add_meme_to_owner(&owner_id, &test_meme_id);

        contract.remove_meme_from_owner(&owner_id, &test_meme_id);
        assert!(contract.memes_per_owner.get(&owner_id).is_none())
    }

    #[test]
    fn remove_meme_from_owner_success_multiple_memes() {
        let owner_id = "bob.testnet".to_string();
        let context = get_context(owner_id.clone());
        testing_env!(context);
        let mut contract = HusyContract::new_default("contract_owner.testnet".to_string());

        let test_meme_id = "test_meme.testnet".to_string();
        let test_meme_id2 = "test_meme2.testnet".to_string();
        contract.add_meme_to_owner(&owner_id, &test_meme_id);
        contract.add_meme_to_owner(&owner_id, &test_meme_id2);

        contract.remove_meme_from_owner(&owner_id, &test_meme_id);
        assert!(contract.memes_per_owner.get(&owner_id).is_some());
        assert!(contract.memes_per_owner.get(&owner_id).unwrap().len() == 1);
    }

    #[test]
    #[should_panic]
    fn remove_meme_from_owner_panics() {
        let owner_id = "some_guy.testnet".to_string();
        let context = get_context(owner_id.clone());
        testing_env!(context);
        let mut contract = HusyContract::new_default("contract_owner.testnet".to_string());

        let test_meme_id = "test_meme.testnet".to_string();

        contract.remove_meme_from_owner(&owner_id, &test_meme_id);
    }
}
