use near_sdk::{env, json_types::U128, AccountId};

use crate::{
    contract::NFTRoyality,
    models::{husy::*, meme::MemeTokenId, payout::Payout},
    utils::payment::with_refund,
};

impl NFTRoyality for HusyContract {
    fn nft_payout(&self, token_id: MemeTokenId, balance: U128, max_len_payout: u32) -> Payout {
        self.get_meme_payout(token_id, balance, max_len_payout)
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
        let sender_id = env::predecessor_account_id();

        with_refund(|| {
            let previous = self.nft_meme_transfer(
                sender_id,
                receiver_id,
                token_id.clone(),
                Some(approval_id),
                memo,
            );
            (
                self.get_meme_payout(token_id, balance, max_len_payout),
                Some(previous.owner_id),
            )
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
        let account_id = "sdafasdfsd.testnet".to_owned();
        let context = get_context(account_id.clone(), 0);
        testing_env!(context);
        let mut contract = HusyContract::new_default("bbb.testnet".to_owned());

        let meme_id = "asvjfljl.testnet".to_owned();
        contract.memes_by_id.insert(
            &meme_id,
            &MemeToken {
                royalty: HashMap::from([
                    ("1.testnet".to_owned(), 100),
                    ("2.testnet".to_owned(), 200),
                ]),
                owner_id: account_id,
                ..Default::default()
            },
        );

        contract.nft_payout(meme_id.clone(), U128(100_000), 1);
    }

    #[test]
    fn nft_payout_success() {
        let account_id = "sdafasdfsd.testnet".to_owned();
        let context = get_context(account_id.clone(), 0);
        testing_env!(context);
        let mut contract = HusyContract::new_default("bbb.testnet".to_owned());

        let meme_id = "asvjfljl.testnet".to_owned();
        contract.memes_by_id.insert(
            &meme_id,
            &MemeToken {
                royalty: HashMap::from([
                    ("1.testnet".to_owned(), 100),
                    ("2.testnet".to_owned(), 200),
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
                    ("1.testnet".to_owned(), U128(1_000)),
                    ("2.testnet".to_owned(), U128(2_000)),
                    (account_id, U128(97_000))
                ]),
            }
        )
    }
}
