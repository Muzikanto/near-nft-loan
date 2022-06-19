use near_sdk::json_types::U128;
use near_sdk::AccountId;

pub trait LoanFactoryStorage {
  fn loan_deposit(&mut self) -> U128;
  fn loan_withdraw(&mut self, amount: U128) -> U128;
  fn loan_withdraw_all(&mut self) -> U128;
  fn loan_claim_rewards(&mut self) -> U128;

  fn loan_balance_of(&self, account_id: AccountId) -> U128;
  fn loan_total_balance(&self) -> U128;
  fn loan_total_loan(&self) -> U128;
  fn loan_total_shares(&self) -> U128;
  fn loan_total_rewards_pool(&self) -> U128;
  fn loan_available_balance(&self) -> U128;
  fn loan_reward_of(&self, account_id: AccountId) -> U128;
  fn loan_shares_of(&self, account_id: AccountId) -> U128;
  fn loan_reward_claimed_of(&self, account_id: AccountId) -> U128;
  fn loan_reward_unclaimed_of(&self, account_id: AccountId) -> U128;
}
