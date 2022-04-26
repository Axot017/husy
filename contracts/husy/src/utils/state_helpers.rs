use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{husy::*, meme::MemeTokenId, storage::StorageKey};

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
}
