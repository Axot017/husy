use near_sdk::{
    borsh::BorshSerialize, collections::UnorderedSet, env, AccountId, Balance, CryptoHash, Promise,
};

use crate::models::{husy::*, meme::MemeTokenId, storage::StorageKey};

pub(crate) fn with_storage_payment(fun: impl FnOnce()) {
    let initial_storage_usage = env::storage_usage();
    fun();
    let required_storage = env::storage_usage() - initial_storage_usage;
    let required_cost = env::storage_byte_cost() * Balance::from(required_storage);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );
    let refund = attached_deposit - required_cost;

    if refund > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

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
}
