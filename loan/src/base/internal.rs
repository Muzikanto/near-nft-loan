use near_sdk::collections::{LookupMap, UnorderedSet};
use std::collections::HashMap;
use near_sdk::{AccountId, env, IntoStorageKey, BorshStorageKey, ONE_YOCTO, Balance};
use near_sdk::json_types::U128;
use crate::base::{ContractId, TokenId, LoanFactory};
use crate::utils::date_now;
use crate::meta::JsonLoan;
use crate::base::base_impl::TIME_IN_WEEK;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  TokensPerOwner { account_hash: Vec<u8> },
}

impl LoanFactory {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
    }
    pub(crate) fn assert_loan_expired(&self, contract_token_id: &TokenId) {
        let expire_date = self.loan_date_by_nft.get(&contract_token_id).expect("Not found loan expire date");

        if expire_date >= date_now() {
            env::panic_str("Loan does not expired");
        }
    }
    pub(crate) fn assert_loan_not_expired(&self, contract_token_id: &TokenId) {
        let expire_date = self.loan_date_by_nft.get(&contract_token_id).expect("Not found loan expire date");

        if expire_date < date_now() {
            env::panic_str("Loan is expired");
        }
    }

    pub(crate) fn internal_get_token_id(&self, contract_id: &ContractId, token_id: &TokenId) -> TokenId {
        format!("{}||{}", contract_id.clone(), token_id.clone())
    }

    // loan balance

    pub(crate) fn internal_increase_loan_balance(&mut self, account_id: &AccountId, amount: &U128) {
        let current = self.loan_by_account.get(&account_id).unwrap_or_else(|| 0);
        let next = current + amount.0;

        self.loan_by_account.insert(&account_id, &next);
    }
    pub(crate) fn internal_decrease_loan_balance(&mut self, account_id: &AccountId, amount: &U128) {
        let current = self.loan_by_account.get(&account_id).unwrap_or_else(|| 0);

        if amount.0 > current {
            env::panic_str(&format!("No funds {} {}", amount.0.to_string(), current.to_string()));
        }

        let next = current - amount.0;

        self.loan_by_account.insert(&account_id, &next);
    }

    pub(crate) fn internal_balance_of_loan(&self, account_id: &AccountId) -> U128 {
        U128::from(self.loan_by_account.get(&account_id).unwrap_or_else(|| 0))
    }

    // loan nft

    pub(crate) fn internal_increase_loan_nft(&mut self, contract_token_id: &TokenId, amount: &U128) {
        let current = self.loan_by_nft.get(&contract_token_id).unwrap_or_else(|| 0);
        let next = current + amount.0;

        self.loan_by_nft.insert(&contract_token_id, &next);
    }
    pub(crate) fn internal_decrease_loan_nft(&mut self, contract_token_id: &TokenId, amount: &U128) {
        let current = self.loan_by_nft.get(&contract_token_id).unwrap_or_else(|| 0);

        if amount.0 > current {
            env::panic_str("No funds");
        }

        let next = current - amount.0;

        self.loan_by_nft.insert(&contract_token_id, &next);
    }

    pub(crate) fn internal_rest_of_loan(&self, contract_token_id: &TokenId) -> U128 {
        U128::from(self.loan_by_nft.get(&contract_token_id).unwrap_or_else(|| 0))
    }

    pub(crate) fn internal_set_nft_price(&mut self, contract_id: &ContractId, price: &Balance, percent: &u64) {
        self.price_by_contract.insert(&contract_id, &price);
        self.percent_by_contract.insert(&contract_id, &percent);
    }
    pub(crate) fn internal_set_loan_expire_date(&mut self, contract_token_id: &TokenId, date: &u64) {
        self.loan_date_by_nft.insert(&contract_token_id, &date);
    }

    pub(crate) fn enum_get_loan(&self, contract_token_id: &TokenId) -> JsonLoan {
      let owner_id = self.owner_by_nft.get(&contract_token_id).expect("Not found token owner");
      let loan = self.loan_by_nft.get(&contract_token_id).expect("Not found loan");
      let expire_date = self.loan_date_by_nft.get(&contract_token_id).unwrap_or_else(|| 0);
      let expired = expire_date < date_now();
      let arr = contract_token_id.split("||").collect::<Vec<&str>>();
      let started_at = if expire_date == 0 {
        0
      } else {
        expire_date - TIME_IN_WEEK
      };

      JsonLoan {
        token_id: arr[1].to_string(),
        contract_id: AccountId::new_unchecked(arr[0].to_string()),
        owner_id,
        price: U128::from(loan),
        started_at,
        expired_at: expire_date,
        expired,
      }
    }

    pub(crate) fn internal_set_nft_owner(&mut self, account_id: &AccountId, contract_token_id: &TokenId) {
      self.owner_by_nft.insert(&contract_token_id, &account_id);

      let mut receiver_tokens = self.nft_by_owner.get(&account_id).unwrap_or_else(|| {
        UnorderedSet::new(StorageKey::TokensPerOwner {
          account_hash: env::sha256(account_id.as_bytes()),
        })
      });
      receiver_tokens.insert(&contract_token_id);
      self.nft_by_owner.insert(&account_id, &receiver_tokens);
    }

    pub(crate) fn internal_remove_nft_owner(&mut self, account_id: &AccountId, contract_token_id: &TokenId) {
      self.owner_by_nft.remove(&contract_token_id);

      let mut owner_tokens = self.nft_by_owner.get(&account_id).unwrap_or_else(|| {
        env::panic_str("Unable to access tokens per owner in unguarded call.")
      });

      owner_tokens.remove(&contract_token_id);

      if owner_tokens.is_empty() {
        self.nft_by_owner.remove(&account_id);
      } else {
        self.nft_by_owner.insert(&account_id, &owner_tokens);
      }
    }
}
