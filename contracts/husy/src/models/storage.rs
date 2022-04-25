use near_sdk::{
    borsh::{self, BorshSerialize},
    CryptoHash,
};

#[derive(BorshSerialize)]
pub enum StorageKey {
    MemesPerOwner,
    MemessById,
    MemeMetadataById,
    HusyContractMetadata,
    MemePerOwnerInner { account_id_hash: CryptoHash },
}
