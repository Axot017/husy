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
    GlobalLikesData,
    MemeAdditionalData,
    MemePerOwnerInner { account_id_hash: CryptoHash },
}
