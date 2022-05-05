use std::collections::HashMap;

use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}
