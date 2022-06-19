use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet, TreeMap};
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, log, require, AccountId, Balance, Gas, IntoStorageKey, PromiseOrValue, PromiseResult, StorageUsage, Promise, is_promise_success};
use crate::base::{LoanFactoryCore, LoanFactoryResolver};
use crate::base::base::{ContractId, TokenId};
use std::collections::HashMap;
use crate::utils::date_now;
use crate::meta::JsonLoan;
use crate::event::{LoanWhitelistUpdatePrice, LoanNftClaimExpired, LoanNftClaim, LoanNft, LoanNftPay};

const CALLBACK_ON_RESOLVE_NFT: Gas = Gas(50_000_000_000_000);
const CALLBACK_ON_PAY: Gas = Gas(20_000_000_000_000);

const GAS_FOR_LOAN_NFT: Gas = Gas(60_000_000_000_000);
const GAS_FOR_LOAN_CLAIM_NFT: Gas = Gas(60_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(18_000_000_000_000);
const NO_DEPOSIT: Balance = 0;
const ONE_YOCTO: Balance = 1;
pub(crate) const TIME_IN_WEEK: u64 = 604800000; // 5 min // 604800000; // 1 week

#[ext_contract(ext_self)]
pub trait ExtSelf {
  fn loan_resolve_nft(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId, price: Balance, percent: u64);
  fn loan_resolve_nft_claim(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId);

  fn on_transfer_nft_pay(&mut self, account_id: AccountId, amount_sent: U128, fee: U128, recipient: AccountId, contract_token_id: TokenId, contract_id: AccountId, token_id: TokenId);
  fn on_transfer_resolve_nft(&mut self, account_id: AccountId, amount_sent: U128, recipient: AccountId, contract_token_id: TokenId, contract_id: AccountId, token_id: TokenId, expire_date: u64);
}

#[ext_contract(ext_nft)]
pub trait NonFungibleTokenLockedReceiver {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct LoanFactory {
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

impl LoanFactory {
    pub fn new<S, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13>(
        owner_id: AccountId,
        commission: u128,
        account_prefix: S,
        loan_by_nft_prefix: S2,
        loan_by_account_prefix: S3,
        nft_owner_prefix: S4,
        price_by_nft_prefix: S5,
        loan_date_by_nft_prefix: S6,
        claim_date_by_nft_prefix: S7,
        reward_by_account_prefix: S8,
        percent_by_nft_prefix: S9,
        nft_by_owner_prefix: S10,
        percent_by_contract_prefix: S11,
        price_by_contract_prefix: S12,
      shares_by_account_prefix: S13
    ) -> Self
        where
            S: IntoStorageKey,
            S2: IntoStorageKey,
            S3: IntoStorageKey,
            S4: IntoStorageKey,
            S5: IntoStorageKey,
            S6: IntoStorageKey,
            S7: IntoStorageKey,
            S8: IntoStorageKey,
            S9: IntoStorageKey,
            S10: IntoStorageKey,
            S11: IntoStorageKey,
            S12: IntoStorageKey,
            S13: IntoStorageKey,
    {
        let mut this = Self {
          total_shares: U128::from(0),
            commission,
            total_rewards_pool: U128::from(0),
            total_loan: U128::from(0),
            total_balance: U128::from(0),
            accounts: TreeMap::new(account_prefix),
            loan_by_nft: LookupMap::new(loan_by_nft_prefix),
            loan_by_account: LookupMap::new(loan_by_account_prefix),
            owner_by_nft: LookupMap::new(nft_owner_prefix),
            price_by_nft: LookupMap::new(price_by_nft_prefix),
            percent_by_nft: LookupMap::new(percent_by_nft_prefix),
            loan_date_by_nft: TreeMap::new(loan_date_by_nft_prefix),
            claim_date_by_account: LookupMap::new(claim_date_by_nft_prefix),
            reward_by_account: LookupMap::new(reward_by_account_prefix),
            nft_by_owner: LookupMap::new(nft_by_owner_prefix),
            percent_by_contract: LookupMap::new(percent_by_contract_prefix),
            price_by_contract: LookupMap::new(price_by_contract_prefix),
            shares_by_account: LookupMap::new(shares_by_account_prefix),
            owner_id,
            whitelist: HashMap::new(),
        };

        this
    }
}

impl LoanFactoryCore for LoanFactory {
    fn loan_nft(&mut self, token_id: TokenId, contract_id: ContractId) {
      self.assert_nft_whitelist(&contract_id);

      let signer_id = env::signer_account_id();
      let receiver_id = env::current_account_id();
      let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);

      let loan = self.loan_by_nft.get(&contract_token_id).unwrap_or_else(|| 0);

      let price = self.price_by_contract.get(&contract_id).expect("Not found price for current nft");
      let percent = self.percent_by_contract.get(&contract_id).expect("Not found percent for current nft");

      self.assert_available_balance(&U128::from(price));

        if loan > 0 {
            env::panic_str("Nft already in loan");
        }

        self.internal_set_nft_owner(&signer_id, &contract_token_id);

        ext_nft::nft_transfer(
            receiver_id,
            token_id.clone(),
            None,
            None,

            contract_id.clone(),
            ONE_YOCTO,
            GAS_FOR_NFT_TRANSFER,
        ).then(ext_self::loan_resolve_nft(
            signer_id.clone(),
            contract_id.clone(),
            token_id.clone(),
            contract_token_id.clone(),
            price.clone(),
            percent.clone(),

            env::current_account_id().clone(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_LOAN_NFT,
        ));
    }

    fn loan_nft_pay(&mut self, token_id: TokenId, contract_id: ContractId) {
      let signer_id = env::predecessor_account_id();
      let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);

      let loan_amount = self.internal_rest_of_loan(&contract_token_id).0;
      let balance = env::attached_deposit();
      let fee = loan_amount * self.commission / 100;
      let return_amount = loan_amount + fee;

      self.assert_loan_not_expired(&contract_token_id);

      if return_amount != balance {
        env::panic_str(&format!("Invalid attached deposit, require {}, current {}", return_amount.to_string(), balance.to_string()));
      }

      self.internal_decrease_loan_nft(&contract_token_id, &U128(loan_amount));
      self.internal_decrease_loan_balance(&signer_id, &U128(loan_amount));
      self.total_loan = U128::from(self.total_loan.0 - loan_amount);
      self.total_rewards_pool = U128::from(self.total_rewards_pool.0 + fee);

      Promise::new(env::current_account_id())
        .transfer(return_amount)
        .then(
        ext_self::on_transfer_nft_pay(
          env::predecessor_account_id(),
          U128::from(return_amount),
          U128::from(fee),
          env::current_account_id(),
          contract_token_id.clone(),
          contract_id.clone(),
          token_id.clone(),
          env::current_account_id(),
          0,
          CALLBACK_ON_PAY,
        )
      );

      // self.total_balance = U128::from(self.total_balance.0 );

      // LoanNftPay {
      //     owner_id: &signer_id,
      //     contract_id: &contract_id,
      //     token_id: &token_id,
      //     loan_amount: &U128::from(loan_amount),
      //   }.emit();

      self.loan_nft_claim(token_id.clone(), contract_id.clone());
    }

    fn loan_nft_claim(&mut self, token_id: TokenId, contract_id: ContractId) {
        let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);
        let receiver_id = self.owner_by_nft.get(&contract_token_id).expect("Not found token owner");
        let loan =self.internal_rest_of_loan(&contract_token_id).0;

        if loan > 0 {
            env::panic_str(&"Close loan first");
        }

        ext_nft::nft_transfer(
            receiver_id.clone(),
            token_id.clone(),
            None,
            None,

            contract_id.clone(),
            ONE_YOCTO,
            GAS_FOR_NFT_TRANSFER,
        ).then(ext_self::loan_resolve_nft_claim(
            receiver_id.clone(),
            contract_id.clone(),
            token_id.clone(),
            contract_token_id.clone(),

            env::current_account_id().clone(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_LOAN_CLAIM_NFT,
        ));
    }

  fn loan_update_nft_price(&mut self, contract_id: ContractId, price: U128, percent: u64) {
    self.assert_owner();

    if percent > (50 as u64) {
      env::panic_str("Max percent is 50");
    }
    if percent < (1 as u64) {
      env::panic_str("Min percent is 1");
    }
    if price.0 < ONE_YOCTO {
      env::panic_str("Invalid price");
    }

    self.internal_set_nft_price(&contract_id, &price.0, &percent);

    LoanWhitelistUpdatePrice {
      contract_id: &contract_id,
      price: &price,
      percent: &percent
    }.emit();
  }

  fn loan_nft_claim_expired(&mut self, token_id: TokenId, contract_id: ContractId) {
    // self.assert_owner();

    let current_id = env::current_account_id();
    let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);
    let owner_id = self.owner_by_nft.get(&contract_token_id).expect("Not found token owner");

    self.assert_loan_expired(&contract_token_id);

    self.internal_remove_nft_owner(&owner_id, &contract_token_id);
    self.internal_set_nft_owner(&current_id, &contract_token_id);
    self.loan_date_by_nft.remove(&contract_token_id);

    LoanNftClaimExpired {
        old_owner_id: &owner_id,
        contract_id: &contract_id,
        token_id: &token_id
      }.emit();
    }

  fn loan_balance_borrowed_of(&self, account_id: AccountId) -> U128 {
      self.internal_balance_of_loan(&account_id)
  }

  fn loan_rest_by_id(&self, token_id: TokenId, contract_id: ContractId) -> U128 {
      let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);

      U128::from(self.loan_by_nft.get(&contract_token_id).expect("Not found loan"))
  }

  fn loan_owner_by_id(&self, token_id: TokenId, contract_id: ContractId) -> AccountId {
      let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);

      self.owner_by_nft.get(&contract_token_id).expect("Not found loan")
  }

  fn loan_nft_price(&self, contract_id: ContractId) -> Vec<U128> {
    let price = self.price_by_contract.get(&contract_id).expect("Not found price");
    let percent = self.percent_by_contract.get(&contract_id).expect("Not found percent");

    vec![U128::from(price), U128::from(percent as u128)]
  }

  fn loan_total_nft(&self) -> u128 {
    self.loan_date_by_nft.len() as u128
  }

  fn loan_commission(&self) -> u128 {
    self.commission
  }

  fn loan_nft_by_id(&self, token_id: TokenId, contract_id: ContractId) -> JsonLoan {
    let contract_token_id = self.internal_get_token_id(&contract_id, &token_id);

    self.enum_get_loan(&contract_token_id)
  }

  fn loan_nft_by_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonLoan> {
    let token_set = if let Some(token_set) = self.nft_by_owner.get(&account_id) {
      token_set
    } else {
      return vec![];
    };
    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    require!(limit != 0, "Cannot provide limit of 0.");
    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    require!(
            token_set.len() as u128 > start_index,
            "Out of bounds, please use a smaller from_index."
        );

    token_set
      .iter()
      .skip(start_index as usize)
      .take(limit)
      .map(|contract_token_id| self.enum_get_loan(&contract_token_id))
      .collect()
  }
}

impl LoanFactoryResolver for LoanFactory {
    fn loan_resolve_nft(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId, price: Balance, percent: u64) {
        let is_success = is_promise_success();

        if is_success {
          let loan_amount = U128::from(price  * ((100 - percent) as u128) / (100 as u128));
          let expire_date = date_now() + TIME_IN_WEEK;

          self.total_loan = U128::from(self.total_loan.0 + loan_amount.0);
          self.internal_increase_loan_nft(&contract_token_id, &loan_amount);
          self.internal_increase_loan_balance(&receiver_id, &loan_amount);
          self.internal_set_loan_expire_date(&contract_token_id, &expire_date);
          // self.owner_by_nft.insert(&contract_token_id, &receiver_id);
          self.price_by_nft.insert(&contract_token_id, &price);
          self.percent_by_nft.insert(&contract_token_id, &percent);

          Promise::new(receiver_id.clone())
            .transfer(loan_amount.0)
            .then(
              ext_self::on_transfer_resolve_nft(
                env::current_account_id(),
                loan_amount.clone(),
                receiver_id.clone(),
                contract_token_id.clone(),
                contract_id.clone(),
                token_id.clone(),
                expire_date.clone(),
                env::current_account_id(),
                0,
                CALLBACK_ON_RESOLVE_NFT,
              ));

          // LoanNft {
          //     owner_id: &receiver_id,
          //     contract_id: &contract_id,
          //     token_id: &token_id,
          //     expire_date: &expire_date,
          //     loan_amount: &loan_amount,
          //     price: &loan_amount,
          //   }.emit();
        } else {
          self.internal_remove_nft_owner(&receiver_id, &contract_token_id);
        }
    }

    fn loan_resolve_nft_claim(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId) {
        let is_success = is_promise_success();

        if is_success {
          self.internal_remove_nft_owner(&receiver_id, &contract_token_id);
          self.loan_date_by_nft.remove(&contract_token_id);
          self.price_by_nft.remove(&contract_token_id);
          self.percent_by_nft.remove(&contract_token_id);

          LoanNftClaim {
            receiver_id: &receiver_id,
            contract_id: &contract_id,
            token_id: &token_id
          }.emit();
        } else {
          // self.internal_remove_loan_owner(&receiver_id, &contract_token_id);
        }
    }
}
