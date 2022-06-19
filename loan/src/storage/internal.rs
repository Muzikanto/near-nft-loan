
use crate::base::LoanFactory;
use near_sdk::{AccountId, env};
use near_sdk::json_types::U128;

impl LoanFactory {
  pub(crate) fn assert_available_balance(&self, amount: &U128) {
    let available_balance = self.internal_available_balance();

    if available_balance.0 < amount.0 {
      env::panic_str(&"Not found available fund");
    }
  }

  pub(crate) fn assert_available_rewards(&self, amount: &U128) {
    let available_balance = self.total_rewards_pool;

    if available_balance.0 < amount.0 {
      env::panic_str(&"Not found available fund for rewards");
    }
  }

  // storage
  pub(crate) fn internal_balance_of(&self, account_id: &AccountId) -> U128 {
      U128::from(self.accounts.get(&account_id).unwrap_or_else(|| 0))
  }
  pub(crate) fn internal_increase_balance(&mut self, account_id: &AccountId, amount: &U128) {
    let current = self.accounts.get(&account_id).unwrap_or_else(|| 0);
    let next = current + amount.0;
    let num_shares = if self.total_balance.0 == 0 {
      U128::from(100)
    } else {
      U128::from(self.total_shares.0 * amount.0 / self.total_balance.0)
    };
    let current_shares = self.shares_by_account.get(&account_id).unwrap_or_else(|| U128::from(0));
    let new_shares = U128::from(num_shares.0 + current_shares.0);

    self.accounts.insert(&account_id, &next);
    self.total_balance = U128::from(self.total_balance.0 + amount.0);

    self.total_shares = U128::from(self.total_shares.0 + num_shares.0);
    self.shares_by_account.insert(&account_id, &new_shares);
  }
  pub(crate) fn internal_decrease_balance(&mut self, account_id: &AccountId, amount: &U128) {
    let current = self.accounts.get(&account_id).unwrap_or_else(|| 0);
    let num_shares = U128::from(self.total_shares.0 * amount.0 / self.total_balance.0);
    let current_shares = self.shares_by_account.get(&account_id).unwrap_or_else(|| U128::from(0));
    let new_shares = U128::from(num_shares.0 - current_shares.0);

    if amount.0 > current {
        env::panic_str("No funds");
    }

    let next = current - amount.0;

    self.accounts.insert(&account_id, &next);
    self.total_balance = U128::from(self.total_balance.0 - amount.0);

    self.total_shares = U128::from(self.total_shares.0 - num_shares.0);
    self.shares_by_account.insert(&account_id, &new_shares);
  }
  pub(crate) fn internal_reward_claimed_of(&self, account_id: &AccountId) -> U128 {
      self.reward_by_account.get(&account_id).unwrap_or_else(|| U128::from(0))
  }
  pub(crate) fn internal_reward_unclaimed_of(&self, account_id: &AccountId) -> U128 {
      let now = env::block_timestamp();
      let claim_date = self.claim_date_by_account.get(&account_id);

    if let Some(num_shares) = self.shares_by_account.get(&account_id) {
      if let Some(claim_date) = claim_date {
        let time_diff = u128::from(now - claim_date);

        if time_diff > 0 {
          let balance = self.internal_balance_of(&account_id);

          if balance.0 == 0 {
            return U128::from(0);
          }

          let total_shares = self.total_shares.0;
          let total_rewards = self.total_rewards_pool.0;
          let rewards = total_rewards * num_shares.0 / total_shares;

          let max_rewards = balance.0 * 28 / 100 / 31536000000000000 * time_diff;

          if rewards > max_rewards {
            return U128::from(max_rewards);
          }

          return U128::from(rewards);
        }
      }
    }

     U128::from(0)
  }
  pub(crate) fn internal_temp_claim(&mut self, account_id: &AccountId) {
      let add_rewards = self.internal_reward_unclaimed_of(&account_id);
      self.claim_date_by_account.insert(&account_id, &env::block_timestamp());

      let prev_rewards = self.reward_by_account.get(&account_id).unwrap_or_else(|| U128::from(0));
      let next_rewards = U128::from(prev_rewards.0 + add_rewards.0);

      self.reward_by_account.insert(&account_id,&next_rewards);
  }
  pub(crate) fn internal_available_balance(&self) -> U128 {
    U128::from(self.total_balance.0 - self.total_loan.0)
  }
}
