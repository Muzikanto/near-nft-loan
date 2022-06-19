use crate::base::{TokenId, ContractId};
use near_sdk::AccountId;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonLoan {
  pub token_id: TokenId,
  pub contract_id: ContractId,
  pub owner_id: AccountId,
  pub started_at: u64,
  pub expired_at: u64,
  pub price: U128,
  pub expired: bool,
}
