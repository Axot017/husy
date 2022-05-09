use near_sdk::env;

use crate::models::{
    global_likes_data::{GlobalLikesData, LikesCountingMode},
    meme_additional_data::MemeAdditionalData,
};

const MAIN_AVERAGE_FACTOR: f64 = 0.8;
const MIN_LIKES: u64 = 3;

const MIN_TIME_TO_SWITCH_MODE: u64 = 604_800_000_000_000;

impl GlobalLikesData {
    fn add_likes(&mut self, likes: u64, count_as_liked: bool) {
        match self.likes_counting_mode {
            LikesCountingMode::FirstGroupActive => {
                self.second_group_sum += likes as u128;
                if count_as_liked {
                    self.second_group_liked_memes += 1;
                }
            }
            _ => {
                self.first_group_sum += likes as u128;
                if count_as_liked {
                    self.first_group_liked_memes += 1;
                }
            }
        }
    }

    fn get_current_average(&self) -> u64 {
        let sum = match self.likes_counting_mode {
            LikesCountingMode::SecondGroupActive => self.second_group_sum,
            _ => self.first_group_sum,
        };
        let memes_count = match self.likes_counting_mode {
            LikesCountingMode::SecondGroupActive => self.second_group_liked_memes,
            _ => self.first_group_liked_memes,
        };
        if memes_count == 0 {
            0
        } else {
            (sum / memes_count as u128) as u64
        }
    }

    pub(crate) fn try_switching_mode(&mut self) {
        let now = env::block_timestamp();
        if now - self.last_group_swap_timestamp > MIN_TIME_TO_SWITCH_MODE {
            self.likes_counting_mode = match self.likes_counting_mode {
                LikesCountingMode::SecondGroupActive => LikesCountingMode::FirstGroupActive,
                _ => LikesCountingMode::SecondGroupActive
            }
        }
    }
}

pub(crate) fn count_new_likes_state(
    meme_likes_state: &mut MemeAdditionalData,
    global_state: &mut GlobalLikesData,
    likes: u64,
) {
    meme_likes_state.likes += likes;

    if !meme_likes_state.showed_on_main {
        let liked_first_time = meme_likes_state.last_counted_like_timestamp
            == global_state.last_group_swap_timestamp;
        global_state.add_likes(likes, liked_first_time);
        if liked_first_time {
            meme_likes_state.last_counted_like_timestamp = global_state.last_group_swap_timestamp;
        }
    }
}

pub(crate) fn try_move_to_main(
    meme_likes_state: &mut MemeAdditionalData,
    global_state: &GlobalLikesData,
) -> bool {
    if meme_likes_state.showed_on_main {
        return false;
    }
    let average = global_state.get_current_average();
    let mut min_likes_amount: u64 = (MAIN_AVERAGE_FACTOR * average as f64).round() as u64;
    if min_likes_amount < MIN_LIKES {
        min_likes_amount = MIN_LIKES;
    }

    if meme_likes_state.likes >= min_likes_amount {
        meme_likes_state.showed_on_main = true;
        return true;
    }

    false
}


