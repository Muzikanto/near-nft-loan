use near_sdk::{env, Balance};

pub(crate) fn date_now() -> u64 {
    env::block_timestamp() / 1000000
}
pub(crate) fn method_disabled() {
  env::panic_str("Method disabled");
}
pub fn yton(yocto_amount: Balance) -> Balance {
  (yocto_amount + (5 * 10u128.pow(23))) / 10u128.pow(24)
}
