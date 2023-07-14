#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use flyconomy_token::*;

/// Most basic PSP22 token.
#[openbrush::contract]
#[allow(clippy::let_unit_value)] // Clippy-specific workaround for errors
pub mod flyconomy_token {
    use openbrush::{
        contracts::ownable::*,
        contracts::psp22::extensions::mintable,
        contracts::psp22::{self, psp22::Internal, Data, PSP22Error},
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct FlyconomyToken {
        #[storage_field]
        psp22: Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl psp22::PSP22 for FlyconomyToken {}

    impl Ownable for FlyconomyToken {}

    impl FlyconomyToken {
        /// Instantiate the contract with `total_supply` tokens of supply.
        ///
        /// All the created tokens will be minted to the caller.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            instance
                .psp22
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
    }

    impl mintable::PSP22Mintable for FlyconomyToken {
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        /// Mints the `amount` of underlying tokens to the recipient identified by the `account` address.
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint_to(account, amount)
        }
    }
}
