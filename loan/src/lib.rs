use near_sdk::{AccountId, Balance, env, log, near_bindgen, PanicOnDefault, PromiseOrValue, BorshStorageKey, assert_self, is_promise_success};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet, TreeMap};
use near_sdk::json_types::U128;use crate::base::LoanFactory;
use std::collections::HashMap;
use crate::utils::yton;
use crate::event::{LoanFtDeposit, LoanFtWithdraw, LoanFtClaimRewards, LoanNftPay, LoanNft};

mod event;
mod base;
mod macros;
mod nft_callback;
mod whitelist;
mod storage;
mod utils;
mod meta;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  BalanceByAccount,
  LoanBalanceByAccount,
  LoanByNft,
  NftContractOwner,
  PriceByNft,
  LoanDateByNft,
  ClaimDateByAccount,
  RewardByAccount,
  PercentByNft,
  NftByOwner,
  PriceByContract,
  PercentByContract,
  SharesByAccount,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  loan: LoanFactory,
}

#[near_bindgen]
impl Contract {
  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// default metadata (for example purposes only).
  #[init]
  pub fn new_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id,
    )
  }

  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// the given fungible token metadata.
  #[init]
  pub fn new(
    owner_id: AccountId,
  ) -> Self {
    assert!(!env::state_exists(), "Already initialized");

    let mut this = Self {
      loan: LoanFactory::new(
        owner_id,
        9,
        StorageKey::BalanceByAccount,
        StorageKey::LoanBalanceByAccount,
        StorageKey::LoanByNft,
        StorageKey::NftContractOwner,
        StorageKey::PriceByNft,
        StorageKey::LoanDateByNft,
        StorageKey::ClaimDateByAccount,
        StorageKey::RewardByAccount,
        StorageKey::PercentByNft,
        StorageKey::NftByOwner,
        StorageKey::PercentByContract,
        StorageKey::PriceByContract,
        StorageKey::SharesByAccount,
      ),
    };

    this
  }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        #[derive(BorshDeserialize, BorshSerialize)]
        pub struct OldLoan {
          pub total_balance: U128,
          pub total_rewards_pool: U128,
          pub total_shares: U128,
          pub total_loan: U128,
          pub commission: u128,
          pub accounts: TreeMap<AccountId, Balance>,
          pub loan_by_nft: LookupMap<TokenId, Balance>,

          pub price_by_nft: LookupMap<TokenId, Balance>,
          pub percent_by_nft: LookupMap<TokenId, u64>,

          pub loan_by_account: LookupMap<AccountId, Balance>,
          pub loan_date_by_nft: TreeMap<TokenId, u64>,
          pub nft_by_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

          pub owner_by_nft: LookupMap<TokenId, AccountId>,

          pub price_by_contract: LookupMap<ContractId, Balance>,
          pub percent_by_contract: LookupMap<ContractId, u64>,
          pub shares_by_account: LookupMap<AccountId, U128>,

          pub claim_date_by_account: LookupMap<AccountId, u64>,
          pub reward_by_account: LookupMap<AccountId, U128>,

          pub whitelist: HashMap<ContractId, bool>,

          pub owner_id: AccountId,
        }
        #[derive(BorshDeserialize)]
        struct Old {
            loan: OldLoan,
        }

        let old: Old = env::state_read().expect("Error");

        let loan = LoanFactory {
          total_shares: U128::from(0),
            commission: old.loan.commission,
            total_rewards_pool: old.loan.total_rewards_pool,
            total_loan: old.loan.total_loan,
            total_balance: old.loan.total_balance,
            accounts: old.loan.accounts,
            loan_by_nft: old.loan.loan_by_nft,
            loan_by_account: old.loan.loan_by_account,
            owner_by_nft: old.loan.owner_by_nft,
            whitelist: old.loan.whitelist,
            owner_id: old.loan.owner_id,
            price_by_contract: old.loan.price_by_contract,
            percent_by_contract: old.loan.percent_by_contract,
            price_by_nft: old.loan.price_by_nft,
            percent_by_nft: old.loan.percent_by_nft,
            shares_by_account: LookupMap::new(StorageKey::SharesByAccount),
            loan_date_by_nft: old.loan.loan_date_by_nft,
            claim_date_by_account: old.loan.claim_date_by_account,
            reward_by_account: old.loan.reward_by_account,
            nft_by_owner: old.loan.nft_by_owner,
        };

        Self {
            loan
        }
    }

  pub fn on_transfer_nft_pay(&mut self, account_id: AccountId, amount_sent: U128, fee: U128, recipient: AccountId, contract_token_id: TokenId, contract_id: AccountId, token_id: TokenId) {
    assert_self();

    let transfer_succeeded = is_promise_success();

    if transfer_succeeded {
      LoanNftPay {
        owner_id: &account_id,
        contract_id: &contract_id,
        token_id: &token_id,
        loan_amount: &U128::from(amount_sent.0),
      }.emit();
    }

    if !transfer_succeeded {
      env::log_str(&format!("Transaction to @{} failed. {} yNEAR (~{} NEAR) kept on the app deposit", recipient, amount_sent.0, yton(amount_sent.0)));

      self.loan.internal_increase_loan_nft(&contract_token_id, &amount_sent);
      self.loan.internal_increase_loan_balance(&account_id, &amount_sent);
      self.loan.total_loan = U128::from(self.loan.total_loan.0 - amount_sent.0);
      self.loan.total_rewards_pool = U128::from(self.loan.total_rewards_pool.0 - fee.0);
    }
  }

  pub fn on_transfer_resolve_nft(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId, contract_token_id: TokenId, contract_id: AccountId, token_id: TokenId, expire_date: u64) {
    assert_self();

    let transfer_succeeded = is_promise_success();

    if transfer_succeeded {
      LoanNft {
        owner_id: &recipient,
        contract_id: &contract_id,
        token_id: &token_id,
        expire_date: &expire_date,
        loan_amount: &amount_sent,
        price: &amount_sent,
      }.emit();
    }

    if !transfer_succeeded {
      env::log_str(&format!("Transaction to @{} failed. {} yNEAR (~{} NEAR) kept on the app deposit", recipient, amount_sent.0, yton(amount_sent.0)));

      self.loan.total_loan = U128::from(self.loan.total_loan.0 - amount_sent.0);
      self.loan.internal_decrease_loan_nft(&contract_token_id, &amount_sent);
      self.loan.internal_decrease_loan_balance(&recipient, &amount_sent);
      self.loan.loan_date_by_nft.remove(&contract_token_id);
      // self.owner_by_nft.insert(&contract_token_id, &receiver_id);
      self.loan.price_by_nft.remove(&contract_token_id);
      self.loan.percent_by_nft.remove(&contract_token_id);
    }
  }

  pub fn on_transfer_loan_deposit(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId) {
    assert_self();

    let transfer_succeeded = is_promise_success();

    if transfer_succeeded {
      LoanFtDeposit {
        account_id: &account_id,
        amount: &U128(amount_sent.0),
      }.emit();
    }

    if !transfer_succeeded {
      env::log_str(&format!("Transaction to @{} failed. {} yNEAR (~{} NEAR) kept on the app deposit", recipient, amount_sent.0, yton(amount_sent.0)));
      self.loan.internal_decrease_balance(&recipient, &amount_sent);
    }
  }

  pub fn on_transfer_loan_withdraw(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId) {
    assert_self();

    let transfer_succeeded = is_promise_success();

    if transfer_succeeded {
      LoanFtWithdraw {
        account_id: &account_id,
        amount: &amount_sent,
      }.emit();
    }

    if !transfer_succeeded {
      env::log_str(&format!("Transaction to @{} failed. {} yNEAR (~{} NEAR) kept on the app deposit", recipient, amount_sent.0, yton(amount_sent.0)));
      self.loan.internal_increase_balance(&recipient, &amount_sent);
    }
  }

  pub fn on_transfer_claim_rewards(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId) {
    assert_self();

    let transfer_succeeded = is_promise_success();

    if transfer_succeeded {
      LoanFtClaimRewards {
        account_id: &account_id,
        amount: &amount_sent,
      }.emit();
    }

    if !transfer_succeeded {
      env::log_str(&format!("Transaction to @{} failed. {} yNEAR (~{} NEAR) kept on the app deposit", recipient, amount_sent.0, yton(amount_sent.0)));
      self.loan.reward_by_account.insert(&recipient, &amount_sent);
      self.loan.total_rewards_pool = U128::from(self.loan.total_rewards_pool.0 - amount_sent.0);
    }
  }
}

// macros

impl_loan_core!(Contract, loan);
impl_loan_storage!(Contract, loan);
impl_loan_whitelist!(Contract, loan);
