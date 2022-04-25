use near_sdk::near_bindgen;

use crate::contract_interface::NFTContractMetadata;
use crate::models::husy::*;
use crate::models::husy_metadata::HusyNFTContractMetadata;

#[near_bindgen]
impl NFTContractMetadata for HusyContract {
    fn nft_metadata(&self) -> HusyNFTContractMetadata {
        self.metadata.get().expect("Failed to get metadata")
    }
}
