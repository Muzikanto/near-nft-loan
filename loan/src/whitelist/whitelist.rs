use crate::base::ContractId;

pub trait LoanFactoryWhitelist {
    fn loan_nft_whitelist_add(&mut self, contract_id: ContractId);
    fn loan_nft_whitelist_remove(&mut self, contract_id: ContractId);
    fn loan_nft_whitelist(&self) -> Vec<ContractId>;
    fn loan_nft_is_whitelist(&self, contract_id: ContractId) -> bool;
}
