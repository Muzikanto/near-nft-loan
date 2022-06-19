use near_sdk::{Balance, AccountId, ext_contract};
use near_sdk::json_types::U128;
use crate::meta::JsonLoan;

pub type TokenId = String;
pub type ContractId = AccountId;

pub trait LoanFactoryCore {
    fn loan_nft(&mut self, token_id: TokenId, contract_id: ContractId);
    fn loan_nft_pay(&mut self, token_id: TokenId, contract_id: ContractId);
    fn loan_nft_claim(&mut self, token_id: TokenId, contract_id: ContractId);
    fn loan_update_nft_price(&mut self, contract_id: ContractId, price: U128, percent: u64);
    fn loan_nft_claim_expired(&mut self, token_id: TokenId, contract_id: ContractId);

    fn loan_balance_borrowed_of(&self, account_id: AccountId) -> U128;
    fn loan_rest_by_id(&self, token_id: TokenId, contract_id: ContractId) -> U128;
    fn loan_owner_by_id(&self, token_id: TokenId, contract_id: ContractId) -> AccountId;

  fn loan_nft_price(&self, contract_id: ContractId) -> Vec<U128>;
  fn loan_total_nft(&self) -> u128;
  fn loan_commission(&self) -> u128;

    fn loan_nft_by_id(&self, token_id: TokenId, contract_id: ContractId) -> JsonLoan;
    fn loan_nft_by_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonLoan>;
}

pub trait LoanFactoryResolver {
    fn loan_resolve_nft(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId, price: Balance, percent: u64);
    fn loan_resolve_nft_claim(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId);
}
