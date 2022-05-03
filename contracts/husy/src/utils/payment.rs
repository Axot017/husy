use std::mem::size_of;

use near_sdk::{env, AccountId, Balance, Promise};

pub(crate) fn with_refund<F, R>(fun: F) -> R
where
    F: FnOnce() -> R,
{
    let initial_storage_usage = env::storage_usage();
    let result = fun();
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

    result 
}
