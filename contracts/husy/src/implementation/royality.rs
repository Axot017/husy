use std::collections::HashMap;

use near_sdk::{json_types::U128, AccountId};

use crate::{
    contract::NFTRoyality,
    models::{husy::*, meme::MemeTokenId, payout::Payout},
    utils::calculation::calculate_procentage,
};

impl NFTRoyality for HusyContract {
    fn nft_payout(&self, token_id: MemeTokenId, balance: U128, max_len_payout: u32) -> Payout {
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

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: MemeTokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use crate::contract::ContractInit;
    use crate::models::meme::MemeToken;

    fn get_context(predecessor_account_id: String, attached: u128) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id.try_into().unwrap())
            .attached_deposit(attached)
            .build()
    }

    #[test]
    #[should_panic]
    fn nft_payout_to_much_payout_accounts() {
        let account_id = "sdafasdfsd.testnet".to_string();
        let context = get_context(account_id.clone(), 0);
        testing_env!(context);
        let mut contract = HusyContract::new_default("bbb.testnet".to_string());

        let meme_id = "asvjfljl.testnet".to_string();
        contract.memes_by_id.insert(
            &meme_id,
            &MemeToken {
                royalty: HashMap::from([
                    ("1.testnet".to_string(), 100),
                    ("2.testnet".to_string(), 200),
                ]),
                owner_id: account_id.clone(),
                ..Default::default()
            },
        );

        contract.nft_payout(meme_id.clone(), U128(100_000), 1);
    }

    #[test]
    fn nft_payout_success() {
        let account_id = "sdafasdfsd.testnet".to_string();
        let context = get_context(account_id.clone(), 0);
        testing_env!(context);
        let mut contract = HusyContract::new_default("bbb.testnet".to_string());

        let meme_id = "asvjfljl.testnet".to_string();
        contract.memes_by_id.insert(
            &meme_id,
            &MemeToken {
                royalty: HashMap::from([
                    ("1.testnet".to_string(), 100),
                    ("2.testnet".to_string(), 200),
                    (account_id.clone(), 900),
                ]),
                owner_id: account_id.clone(),
                ..Default::default()
            },
        );

        let result = contract.nft_payout(meme_id.clone(), U128(100_000), 10);

        assert_eq!(
            result,
            Payout {
                payout: HashMap::from([
                    ("1.testnet".to_string(), U128(1_000)),
                    ("2.testnet".to_string(), U128(2_000)),
                    (account_id, U128(97_000))
                ]),
            }
        )
    }
}
