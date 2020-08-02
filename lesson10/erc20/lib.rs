#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;
    use ink_core::env::AccountId;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Erc20 {
        /// 总供应量
        total_supply: storage::Value<Balance>,
        /// 每个账户的余额
        balances: storage::HashMap<AccountId, Balance>,
    }

    /// 合约初始化事件
    #[ink(event)]
    struct ContractNew {
        #[ink(topic)]
        creator: Option<AccountId>,
    }

    /// 转移事件
    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    impl Erc20 {

        #[ink(constructor)]
        fn new(&mut self, init_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(init_supply);
            self.balances.insert(caller, &self.total_supply);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: &self.total_supply,
            });
            self.env().emit_event(ContractNew {
                creator: caller
            })
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
           *self.balances.get(owner).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            self.transfer_from_to(caller, to ,value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_account_balance = self.balance_of_or_zero(&from);
            if from_account_balance < value {
                false
            }
            // from账户扣减
            self.balances.insert(from, from_account_balance - value);
            // to账户增加
            let to_account_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_account_balance + value);

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            true
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            // Note that even though we defined our `#[ink(constructor)]`
            // above as `&mut self` functions that return nothing we can call
            // them in test code as if they were normal Rust constructors
            // that take no `self` argument but return `Self`.
            let erc20 = Erc20::default();
            assert_eq!(erc20.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut erc20 = Erc20::new(false);
            assert_eq!(erc20.get(), false);
            erc20.flip();
            assert_eq!(erc20.get(), true);
        }
    }
}
