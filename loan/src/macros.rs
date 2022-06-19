/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_loan_core {
    ($contract: ident, $token: ident) => {
        use $crate::base::{LoanFactoryCore, LoanFactoryResolver};
        use $crate::base::{ContractId, TokenId};
        use $crate::meta::{JsonLoan};

        #[near_bindgen]
        impl LoanFactoryCore for $contract {
            fn loan_balance_borrowed_of(&self, account_id: AccountId) -> U128 {
                self.$token.loan_balance_borrowed_of(account_id)
            }

            #[payable]
            fn loan_nft(&mut self, token_id: TokenId, contract_id: ContractId) {
                self.$token.loan_nft(token_id, contract_id)
            }

            fn loan_owner_by_id(&self, token_id: TokenId, contract_id: ContractId) -> AccountId {
                self.$token.loan_owner_by_id(token_id, contract_id)
            }

            fn loan_rest_by_id(&self, token_id: TokenId, contract_id: ContractId) -> U128 {
                self.$token.loan_rest_by_id(token_id, contract_id)
            }

            #[payable]
            fn loan_nft_pay(&mut self, token_id: TokenId, contract_id: ContractId) {
                self.$token.loan_nft_pay(token_id, contract_id)
            }

            fn loan_nft_claim(&mut self, token_id: TokenId, contract_id: ContractId) {
                self.$token.loan_nft_claim(token_id, contract_id)
            }

            fn loan_update_nft_price(&mut self, contract_id: ContractId, price: U128, percent: u64) {
                self.$token.loan_update_nft_price(contract_id, price, percent)
            }
            fn loan_nft_price(&self, contract_id: ContractId) -> Vec<U128> {
                self.$token.loan_nft_price(contract_id)
            }
            fn loan_nft_claim_expired(&mut self, token_id: TokenId, contract_id: ContractId) {
                self.$token.loan_nft_claim_expired(token_id, contract_id)
            }

            fn loan_nft_by_id(&self, token_id: TokenId, contract_id: ContractId) -> JsonLoan {
                self.$token.loan_nft_by_id(token_id, contract_id)
            }
            fn loan_nft_by_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonLoan> {
                self.$token.loan_nft_by_owner(account_id, from_index, limit)
            }

             fn loan_total_nft(&self) -> u128 {
                self.$token.loan_total_nft()
            }
            fn loan_commission(&self) -> u128 {
                self.$token.loan_commission()
            }
        }

        #[near_bindgen]
        impl LoanFactoryResolver for $contract {
            fn loan_resolve_nft(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId, price: Balance, percent: u64) {
                self.$token.loan_resolve_nft(receiver_id, contract_id, token_id, contract_token_id, price, percent)
            }
            fn loan_resolve_nft_claim(&mut self, receiver_id: AccountId, contract_id: ContractId, token_id: TokenId, contract_token_id: TokenId) {
                self.$token.loan_resolve_nft_claim(receiver_id, contract_id, token_id, contract_token_id)
            }
        }
    };
}

/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_loan_whitelist {
    ($contract: ident, $token: ident) => {
        use $crate::whitelist::{LoanFactoryWhitelist};

        #[near_bindgen]
        impl LoanFactoryWhitelist for $contract {
            #[payable]
            fn loan_nft_whitelist_add(&mut self, contract_id: ContractId) {
                self.$token.loan_nft_whitelist_add(contract_id)
            }
            #[payable]
            fn loan_nft_whitelist_remove(&mut self, contract_id: ContractId) {
                self.$token.loan_nft_whitelist_remove(contract_id)
            }

            fn loan_nft_whitelist(&self) -> Vec<ContractId> {
                self.$token.loan_nft_whitelist()
            }
            fn loan_nft_is_whitelist(&self, contract_id: ContractId) -> bool {
                self.$token.loan_nft_is_whitelist(contract_id)
            }
        }
    };
}

/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_loan_storage {
    ($contract: ident, $token: ident) => {
        use $crate::storage::{LoanFactoryStorage};

        #[near_bindgen]
        impl LoanFactoryStorage for $contract {
            #[payable]
            fn loan_deposit(&mut self) -> U128 {
                self.$token.loan_deposit()
            }
            fn loan_withdraw(&mut self, amount: U128) -> U128 {
                self.$token.loan_withdraw(amount)
            }
            fn loan_withdraw_all(&mut self) -> U128 {
                self.$token.loan_withdraw_all()
            }
            fn loan_balance_of(&self, account_id: AccountId) -> U128 {
                self.$token.loan_balance_of(account_id)
            }
            fn loan_shares_of(&self, account_id: AccountId) -> U128 {
                self.$token.loan_shares_of(account_id)
            }
            fn loan_reward_of(&self, account_id: AccountId) -> U128 {
                self.$token.loan_reward_of(account_id)
            }
            fn loan_reward_unclaimed_of(&self, account_id: AccountId) -> U128 {
                self.$token.loan_reward_unclaimed_of(account_id)
            }
            fn loan_reward_claimed_of(&self, account_id: AccountId) -> U128 {
                self.$token.loan_reward_claimed_of(account_id)
            }
             fn loan_claim_rewards(&mut self) -> U128 {
                self.$token.loan_claim_rewards()
            }
            fn loan_total_balance(&self) -> U128 {
              self.$token.loan_total_balance()
            }
             fn loan_total_shares(&self) -> U128 {
              self.$token.loan_total_shares()
            }
            fn loan_total_rewards_pool(&self) -> U128 {
              self.$token.loan_total_rewards_pool()
            }
            fn loan_available_balance(&self) -> U128 {
              self.$token.loan_available_balance()
            }
            fn loan_total_loan(&self) -> U128 {
              self.$token.loan_total_loan()
            }
        }
    };
}
