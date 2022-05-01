use near_sdk::env;

pub(crate) fn assert_full_access_key() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}
