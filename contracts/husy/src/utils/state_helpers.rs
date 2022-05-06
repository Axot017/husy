use std::collections::HashMap;

use near_sdk::{
    assert_one_yocto, borsh::BorshSerialize, collections::UnorderedSet, env, json_types::U128,
    AccountId,
};

use crate::{
    models::{
        husy::*,
        meme::{MemeToken, MemeTokenId, MemeTokenView},
        meme_metadata::MemeTokenMetadata,
        payout::Payout,
        storage::StorageKey,
    },
    utils::calculation::calculate_procentage,
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
            approved_account_ids: token.approved_account_ids,
            royalty: token.royalty,
            likes: token.likes,
            category: token.category,
            showed_on_main: token.showed_on_main,
        });
    }

    pub(crate) fn nft_meme_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> MemeToken {
        assert_one_yocto();

        let token = self
            .memes_by_id
            .get(&token_id)
            .expect("Token id is invalid");

        if token.owner_id != sender_id {
            if !token.approved_account_ids.contains_key(&sender_id) {
                panic!("Unauthorized")
            }
            match token.approved_account_ids.get(&sender_id) {
                Some(expected_approval_id) => {
                    if let Some(approval_id) = approval_id {
                        assert_eq!(&approval_id, expected_approval_id, "Sender is not approved")
                    }
                }
                None => panic!("Unauthorized"),
            }
        }

        assert_eq!(token.owner_id, sender_id, "Unauthorized");
        assert_ne!(
            receiver_id, sender_id,
            "Owner and recievers should be different",
        );

        self.swap_meme_owner(&sender_id, &receiver_id, &token_id);

        self.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: receiver_id,
                next_approval_id: token.next_approval_id,
                ..Default::default()
            },
        );

        if let Some(memo) = memo {
            env::log(format!("Memo: {}", memo).as_bytes());
        }

        token // Token before transfer
    }

    pub(crate) fn get_meme_payout(
        &self,
        token_id: MemeTokenId,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        let token = self.memes_by_id.get(&token_id).expect("Invalid token id");
        assert!(
            token.royalty.len() as u32 + 1 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        let mut payout: HashMap<String, U128> = token
            .royalty
            .iter()
            .filter(|(key, _a)| *key != &token.owner_id)
            .map(|(key, value)| {
                (
                    key.to_owned(),
                    U128(calculate_procentage(value.to_owned(), balance.into())),
                )
            })
            .collect();

        let owner_payout = balance.0 - payout.values().map(|value| value.0).sum::<u128>();

        payout.insert(token.owner_id, U128(owner_payout));

        Payout { payout }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
            approved_account_ids: HashMap::from([("approved.testnet".to_string(), 0)]),
            next_approval_id: 1,
            royalty: HashMap::from([("royality.testnet".to_string(), 1000)]),
            likes: 1,
            showed_on_main: true,
            last_counted_like_timestamp: 0,
            category: Some("category".to_string()),
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
                approved_account_ids: HashMap::from([("approved.testnet".to_string(), 0)]),
                royalty: HashMap::from([("royality.testnet".to_string(), 1000)]),
                likes: 1,
                showed_on_main: true,
                category: Some("category".to_string()),
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
            ..Default::default()
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
            ..Default::default()
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
                ..Default::default()
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
