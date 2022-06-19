use crate::base::{LoanFactory, ContractId};
use crate::whitelist::LoanFactoryWhitelist;
use near_sdk::env;
use crate::event::{LoanWhitelistAdd, LoanWhitelistRemove};

impl LoanFactoryWhitelist for LoanFactory {
    fn loan_nft_whitelist_add(&mut self, contract_id: ContractId) {
        self.assert_owner();
        // let price = self.price_by_contract.get(&contract_id).is_some();

        // if !price {
        //     env::panic_str("Please setup price first");
        // }
        // let percent = self.percent_by_contract.get(&contract_id).is_some();
        //
        // if !percent {
        //     env::panic_str("Please setup percent first");
        // }

        self.whitelist.insert(contract_id.clone(), true);

        LoanWhitelistAdd {
          contract_id: &contract_id,
        }.emit();
    }
    fn loan_nft_whitelist_remove(&mut self, contract_id: ContractId) {
        self.assert_owner();
        self.whitelist.remove(&contract_id);
        self.price_by_contract.remove(&contract_id);
        self.percent_by_contract.remove(&contract_id);

      LoanWhitelistRemove {
        contract_id: &contract_id,
      }.emit();
    }
    fn loan_nft_whitelist(&self) -> Vec<ContractId> {
        self.whitelist
            .iter()
            .filter(|el | {
              self.internal_is_nft_whitelist(el.0)
            })
            .map(|key| key.0.clone())
            .collect()
    }
    fn loan_nft_is_whitelist(&self, contract_id: ContractId) -> bool {
        self.internal_is_nft_whitelist(&contract_id)
    }
}
