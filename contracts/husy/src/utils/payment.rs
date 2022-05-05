use std::{collections::HashMap, mem::size_of};

use near_sdk::{env, AccountId, Balance, Promise};

pub(crate) fn with_refund<F, R>(fun: F, reveiver_id: Option<AccountId>) -> R
where
    F: FnOnce() -> R,
{
    let initial_storage_usage = env::storage_usage();
    let result = fun();
    let final_storage_usage = env::storage_usage();
    let refund = if initial_storage_usage > final_storage_usage {
        let released_storage = initial_storage_usage - final_storage_usage;
        env::storage_byte_cost() * Balance::from(released_storage)
    } else {
        let required_additional_storage = final_storage_usage - initial_storage_usage;
        let required_cost = env::storage_byte_cost() * Balance::from(required_additional_storage);
        let attached_deposit = env::attached_deposit();
        assert!(
            required_cost <= attached_deposit,
            "Must attach {} yoctoNEAR to cover storage",
            required_cost,
        );
        attached_deposit - required_cost
    };

    if refund > 0 {
        let receiver_id = reveiver_id.unwrap_or(env::predecessor_account_id());
        Promise::new(receiver_id).transfer(refund);
    }

    result
}

pub(crate) fn bytes_for_account_id(account_id: &AccountId) -> u64 {
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

pub(crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
    let storage_released: u64 = approved_account_ids.keys().map(bytes_for_account_id).sum();
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}
