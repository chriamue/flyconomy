#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Mintable)]
#[openbrush::contract]
pub mod flyconomy_token {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct FlyconomyToken {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl FlyconomyToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22::psp22_external::PSP22;
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        type ContractRef = FlyconomyTokenRef;

        use test_helpers::{address_of, balance_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("flyconomy_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(alice)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("flyconomy_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 50, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            let balance_of_bob = balance_of!(client, address, bob);

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("flyconomy_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 101, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(
                result.return_value(),
                Err(PSP22Error::InsufficientBalance)
            ));

            Ok(())
        }
    }
}
