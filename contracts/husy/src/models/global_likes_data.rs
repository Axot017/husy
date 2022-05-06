use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Clone, Debug)]
pub enum LikesCountingMode {
    Initial,
    FirstGroupActive,
    SecondGroupActive,
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Clone, Debug)]
pub struct GlobalLikesData {
    pub first_group_sum: u128,
    pub first_group_liked_memes: u64,
    pub second_group_sum: u128,
    pub second_group_liked_memes: u64,
    pub last_group_swap_timestamp: u64,
    pub likes_counting_mode: LikesCountingMode,
}

impl GlobalLikesData {
    pub fn new() -> Self {
        GlobalLikesData {
            first_group_liked_memes: 0,
            first_group_sum: 0,
            second_group_liked_memes: 0,
            second_group_sum: 0,
            likes_counting_mode: LikesCountingMode::Initial,
            last_group_swap_timestamp: env::block_timestamp(),
        }
    }
}
