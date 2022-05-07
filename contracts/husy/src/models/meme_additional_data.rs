use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Clone, Default, Debug)]
pub struct MemeAdditionalData {
    pub likes: u64,
    pub showed_on_main: bool,
    pub last_counted_like_timestamp: u64,
    pub category: Option<String>,
}
