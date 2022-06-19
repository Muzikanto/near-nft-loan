use crate::base::LoanFactory;
use crate::storage::LoanFactoryStorage;
use near_sdk::json_types::U128;
use near_sdk::{AccountId, env, Promise, Gas, ext_contract};
use crate::utils::method_disabled;
use crate::event::{LoanFtDeposit, LoanFtWithdraw, LoanFtClaimRewards};

pub const CALLBACK_ON_DEPOSIT: Gas = Gas(50_000_000_000_000);

#[ext_contract(ext_self)]
pub trait ExtSelf {
  fn on_transfer_loan_deposit(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId);
  fn on_transfer_loan_withdraw(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId);
  fn on_transfer_claim_rewards(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId);
}

impl LoanFactoryStorage for LoanFactory {
    fn loan_deposit(&mut self) -> U128 {
        let account_id = env::predecessor_account_id();
        let balance = env::attached_deposit();

        self.internal_temp_claim(&account_id);

        self.internal_increase_balance(&account_id, &U128::from(balance));

        Promise::new(env::current_account_id())
          .transfer(balance)
          .then(
            ext_self::on_transfer_loan_deposit(
              env::predecessor_account_id(),
              U128::from(balance),
              env::current_account_id(),
              env::current_account_id(),
              0,
              CALLBACK_ON_DEPOSIT,
            )
          );

        // LoanFtDeposit {
        //   account_id: &account_id,
        //   amount: &U128(balance),
        // }.emit();

        self.internal_balance_of(&account_id)
    }
    fn loan_withdraw(&mut self, amount: U128) -> U128 {
      let account_id = env::predecessor_account_id();

      self.assert_available_balance(&amount);

      self.internal_temp_claim(&account_id);
      // self.loan_claim_rewards();

      self.internal_decrease_balance(&account_id, &amount);
      Promise::new(account_id.clone())
        .transfer(amount.0)
        .then(
          ext_self::on_transfer_loan_withdraw(
            env::current_account_id(),
            amount.clone(),
            account_id.clone(),
            env::current_account_id(),
            0,
            CALLBACK_ON_DEPOSIT,
          )
        );

      // LoanFtWithdraw {
      //   account_id: &account_id,
      //   amount: &amount,
      // }.emit();

      amount
    }
    fn loan_withdraw_all(&mut self) -> U128 {
      let account_id = env::predecessor_account_id();
      let balance = self.internal_balance_of(&account_id);

      self.loan_withdraw(balance)
    }
  fn loan_claim_rewards(&mut self) -> U128 {
    let account_id = env::predecessor_account_id();

    self.internal_temp_claim(&account_id);

    let amount = self.loan_reward_of(account_id.clone());

    self.assert_available_rewards(&amount);

    self.reward_by_account.insert(&account_id, &U128::from(0));
    self.total_rewards_pool = U128::from(self.total_rewards_pool.0 - amount.0);

    Promise::new(account_id.clone())
      .transfer(amount.0)
      .then(
      ext_self::on_transfer_claim_rewards(
        env::current_account_id(),
        amount.clone(),
        account_id.clone(),
        env::current_account_id(),
        0,
        CALLBACK_ON_DEPOSIT,
      )
    );

    // LoanFtClaimRewards {
    //   account_id: &account_id,
    //   amount: &amount,
    // }.emit();

    amount
  }

  fn loan_balance_of(&self, account_id: AccountId) -> U128 {
      self.internal_balance_of(&account_id)
  }

  fn loan_total_balance(&self) -> U128 {
    self.total_balance
  }

  fn loan_total_loan(&self) -> U128 {
     self.total_loan
  }

  fn loan_total_shares(&self) -> U128 {
    self.total_shares
  }

  fn loan_total_rewards_pool(&self) -> U128 {
    self.total_rewards_pool
  }

  fn loan_available_balance(&self) -> U128 {
    self.internal_available_balance()
  }

  fn loan_reward_of(&self, account_id: AccountId) -> U128 {
      let unclaimed = self.internal_reward_unclaimed_of(&account_id);
      let claimed = self.internal_reward_claimed_of(&account_id);

      U128::from(unclaimed.0 + claimed.0)
  }

  fn loan_shares_of(&self, account_id: AccountId) -> U128 {
    self.shares_by_account.get(&account_id).unwrap_or_else(|| U128::from(0))
  }

  fn loan_reward_claimed_of(&self, account_id: AccountId) -> U128 {
      self.internal_reward_claimed_of(&account_id)
  }

  fn loan_reward_unclaimed_of(&self, account_id: AccountId) -> U128 {
     self.internal_reward_unclaimed_of(&account_id)
  }
}
