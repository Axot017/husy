use near_sdk::{env, near_bindgen, AccountId, Balance, Gas};

use crate::{
    contract::NFTApproval,
    ext_contracts::ext_nft_approval_receiver,
    models::{husy::*, meme::MemeTokenId},
    utils::{asserts::assert_full_access_key, payment::with_refund},
};

const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl NFTApproval for HusyContract {
    #[payable]
    fn nft_approve(&mut self, token_id: MemeTokenId, account_id: AccountId, msg: Option<String>) {
        assert_full_access_key();

        let mut token = self.memes_by_id.get(&token_id).expect("");

        assert_eq!(
            &token.owner_id,
            &env::predecessor_account_id(),
            "Predecessor must be token owner"
        );

        let approval_id: u64 = token.next_approval_id;
        let approved_account_id = account_id.clone();
        with_refund(|| {
            token
                .approved_account_ids
                .insert(approved_account_id, approval_id);

            token.next_approval_id += 1;
            self.memes_by_id.insert(&token_id, &token);

            ((), None)
        });

        if let Some(msg) = msg {
            ext_nft_approval_receiver::nft_on_approve(
                token_id,
                token.owner_id,
                approval_id,
                msg,
                &account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            )
            .as_return();
        }
    }

    fn nft_is_approved(
        &self,
        token_id: MemeTokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        let token = self.memes_by_id.get(&token_id).expect("Invalid token id");

        match (
            token.approved_account_ids.get(&approved_account_id),
            approval_id,
        ) {
            (Some(expected_approval_id), Some(approval_id)) => expected_approval_id == &approval_id,
            (Some(_), None) => true,
            (None, _) => false,
        }
    }

    fn nft_revoke(&mut self, token_id: MemeTokenId, account_id: AccountId) {
        let mut token = self.memes_by_id.get(&token_id).expect("Invalid token id");
        assert_eq!(
            &token.owner_id,
            &env::predecessor_account_id(),
            "Unauthorized"
        );

        with_refund(|| {
            token.approved_account_ids.remove(&account_id);
            self.memes_by_id.insert(&token_id, &token);

            ((), None)
        });
    }

    fn nft_revoke_all(&mut self, token_id: MemeTokenId) {
        let mut token = self.memes_by_id.get(&token_id).expect("Invalid token id");

        assert_eq!(
            &token.owner_id,
            &env::predecessor_account_id(),
            "Unauthorized"
        );

        with_refund(|| {
            token.approved_account_ids.clear();
            self.memes_by_id.insert(&token_id, &token);

            ((), None)
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::contract::ContractInit;
    use crate::models::meme::MemeToken;

    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    fn get_context(predecessor_account_id: &str, attached_deposit: u128) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id.try_into().unwrap())
            .attached_deposit(attached_deposit)
            .build()
    }

    #[test]
    fn nft_revoke_all_success() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(account_id.clone());
        let token_id = "asdfzvczx.testnet".to_string();
        contract.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: account_id.clone(),
                approved_account_ids: HashMap::from([
                    ("account1.testnet".to_string(), 0),
                    ("account2.testnet".to_string(), 1),
                    ("account3.testnet".to_string(), 2),
                    ("account4.testnet".to_string(), 3),
                ]),
                next_approval_id: 4,
                ..Default::default()
            },
        );

        contract.nft_revoke_all(token_id.clone());

        assert!(contract
            .memes_by_id
            .get(&token_id)
            .unwrap()
            .approved_account_ids
            .is_empty());
    }

    #[test]
    #[should_panic]
    fn nft_revoke_all_panic_when_unauthorized() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(account_id.clone());
        let token_id = "asdfzvczx.testnet".to_string();
        contract.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: "unauthorized.testnet".to_string(),
                ..Default::default()
            },
        );

        contract.nft_revoke_all(token_id)
    }

    #[test]
    #[should_panic]
    fn nft_revoke_all_panic_when_invalid_token_id() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(account_id.clone());

        contract.nft_revoke_all("invalid_token_id.testnet".to_string())
    }

    #[test]
    fn nft_revoke_success() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let approved_account_id = "approved.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(account_id.clone());
        let token_id = "asdfzvczx.testnet".to_string();
        contract.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: account_id.clone(),
                approved_account_ids: HashMap::from([(approved_account_id.clone(), 0)]),
                next_approval_id: 1,
                ..Default::default()
            },
        );

        contract.nft_revoke(token_id.clone(), approved_account_id);

        assert!(contract
            .memes_by_id
            .get(&token_id)
            .unwrap()
            .approved_account_ids
            .is_empty());
    }

    #[test]
    #[should_panic]
    fn nft_revoke_panic_when_unauthorized() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(account_id.clone());
        let token_id = "asdfzvczx.testnet".to_string();
        contract.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: "unauthorized.testnet".to_string(),
                ..Default::default()
            },
        );

        contract.nft_revoke(token_id, "some_account.testnet".to_string())
    }

    #[test]
    #[should_panic]
    fn nft_revoke_panic_when_invalid_token_id() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(account_id.clone());

        contract.nft_revoke(
            "invalid_token_id.testnet".to_string(),
            "some_account.testnet".to_string(),
        )
    }

    #[test]
    #[should_panic]
    fn nft_is_approved_panic() {
        let account_id = "acbvbcvbc.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let contract = HusyContract::new_default(account_id.clone());

        contract.nft_is_approved(
            "some_token.testnet".to_string(),
            "some_account".to_string(),
            None,
        );
    }

    #[test]
    fn nft_is_approved_success() {
        let account_id = "asdfasd.testnet".to_string();
        let owner_id = "asdfasdfasdffds.testnet".to_string();
        let ctx = get_context(&account_id, 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default(owner_id.clone());

        contract.memes_by_id.insert(
            &"not_approved.testnet".to_string(),
            &MemeToken {
                approved_account_ids: HashMap::new(),
                next_approval_id: 0,
                owner_id: owner_id.clone(),
                ..Default::default()
            },
        );
        contract.memes_by_id.insert(
            &"approved.testnet".to_string(),
            &MemeToken {
                approved_account_ids: HashMap::from([(account_id.clone(), 2)]),
                next_approval_id: 3,
                owner_id: owner_id.clone(),
                ..Default::default()
            },
        );

        assert_eq!(
            contract.nft_is_approved("not_approved.testnet".to_string(), account_id.clone(), None),
            false
        );
        assert_eq!(
            contract.nft_is_approved(
                "approved.testnet".to_string(),
                "not_owner.testnet".to_string(),
                None
            ),
            false
        );
        assert_eq!(
            contract.nft_is_approved("approved.testnet".to_string(), account_id.clone(), Some(1)),
            false
        );
        assert_eq!(
            contract.nft_is_approved("approved.testnet".to_string(), account_id.clone(), None),
            true
        );
        assert_eq!(
            contract.nft_is_approved("approved.testnet".to_string(), account_id.clone(), Some(2)),
            true
        );
    }

    #[test]
    #[should_panic]
    fn nft_approve_panics_when_no_deposit() {
        let ctx = get_context("test.testnet", 0);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default("test.testnet".to_string());

        contract.nft_approve("something".to_string(), "something".to_string(), None);
    }

    #[test]
    #[should_panic]
    fn nft_approve_panics_user_not_owner() {
        let user_id = "terstes.testnet";
        let ctx = get_context(user_id, 1000);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default("test.testnet".to_string());
        let token_id = "qweqw.testnet".to_string();
        contract.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: "invalid_owner".to_string(),
                ..Default::default()
            },
        );

        contract.nft_approve(token_id, "something".to_string(), None);
    }

    #[test]
    fn nft_approve_success() {
        let user_id = "terstes.testnet";
        let ctx = get_context(user_id, 10000000000000000000000);
        testing_env!(ctx);
        let mut contract = HusyContract::new_default("test.testnet".to_string());
        let token_id = "qweqw.testnet".to_string();
        let approved_account_id = "something".to_string();

        contract.memes_by_id.insert(
            &token_id,
            &MemeToken {
                owner_id: user_id.to_string(),
                ..Default::default()
            },
        );

        contract.nft_approve(token_id.clone(), approved_account_id.clone(), None);

        let meme = contract.memes_by_id.get(&token_id).unwrap();
        let mut expected_map = HashMap::new();
        expected_map.insert(approved_account_id, 0);
        assert_eq!(meme.approved_account_ids, expected_map);
        assert_eq!(meme.next_approval_id, 1);
    }
}
