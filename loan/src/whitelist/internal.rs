use crate::base::{ContractId, LoanFactory};
use near_sdk::env;

impl LoanFactory {
    pub(crate) fn assert_nft_whitelist(&self, contract_id: &ContractId) {
        let is_contain = self.internal_is_nft_whitelist(&contract_id);

        if is_contain != true {
            env::panic_str("Nft not allowed");
        }
    }
    pub(crate) fn internal_is_nft_whitelist(&self, contract_id: &ContractId) -> bool {
      let exists = self.whitelist.contains_key(contract_id);
      let price = self.price_by_contract.get(&contract_id).is_some();
      let percent = self.percent_by_contract.get(&contract_id).is_some();

      exists && price && percent
    }
}
