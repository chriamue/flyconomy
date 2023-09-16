#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Burnable, PSP34Enumerable)]
#[openbrush::contract]
pub mod flyconomy_attractions {
    use openbrush::traits::Storage;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Attraction {
        id: Id,
        lat: i32, // multiplied by 10,000 for precision
        lon: i32, // multiplied by 10,000 for precision
        name: ink::prelude::string::String,
        description: ink::prelude::string::String,
    }

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        enumerable: enumerable::Data,
        attractions: ink::storage::Mapping<Id, Attraction>,
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, account_id: AccountId, id: Id) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, account_id, id.clone())?;
            self.attractions.insert(
                id.clone(),
                &Attraction {
                    id: id,
                    lat: 0,
                    lon: 0,
                    name: ink::prelude::string::String::from(""),
                    description: ink::prelude::string::String::from(""),
                },
            );
            Ok(())
        }

        #[ink(message)]
        pub fn set_location(&mut self, id: Id, lat: i32, lon: i32) -> Result<(), PSP34Error> {
            let mut attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            attraction.lat = lat;
            attraction.lon = lon;
            self.attractions.insert(id, &attraction);
            Ok(())
        }

        #[ink(message)]
        pub fn get_location(&self, id: Id) -> Result<(i32, i32), PSP34Error> {
            let attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            Ok((
                attraction.lat,
                attraction.lon,
            ))
        }

        #[ink(message)]
        pub fn set_name(&mut self, id: Id, name: ink::prelude::string::String) -> Result<(), PSP34Error> {
            let mut attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            attraction.name = name;
            self.attractions.insert(id, &attraction);
            Ok(())
        }

        #[ink(message)]
        pub fn get_name(&self, id: Id) -> Result<ink::prelude::string::String, PSP34Error> {
            let attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            Ok(attraction.name.clone())
        }

        #[ink(message)]
        pub fn set_description(&mut self, id: Id, description: ink::prelude::string::String) -> Result<(), PSP34Error> {
            let mut attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            attraction.description = description;
            self.attractions.insert(id, &attraction);
            Ok(())
        }

        #[ink(message)]
        pub fn get_description(&self, id: Id) -> Result<ink::prelude::string::String, PSP34Error> {
            let attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            Ok(attraction.description.clone())
        }

        #[ink(message)]
        pub fn update_token(&mut self, id:Id, name: ink::prelude::string::String, description: ink::prelude::string::String, lat: i32, lon: i32) -> Result<(), PSP34Error> {
            let mut attraction = self.attractions.get(&id).ok_or(PSP34Error::TokenNotExists)?;
            attraction.name = name;
            attraction.description = description;
            attraction.lat = lat;
            attraction.lon = lon;
            self.attractions.insert(id, &attraction);
            Ok(())
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::flyconomy_attractions::PSP34Impl;
        use test_helpers::address_of;

        #[ink::test]
        fn mint_works() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
        }

        #[ink::test]
        fn mint_existing_should_fail() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
            assert!(matches!(
                contract.mint(address_of!(Alice), Id::U8(1)),
                Err(_)
            ));
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
        }

        #[ink::test]
        fn attractions_works() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
            let attraction = contract.attractions.get(&Id::U8(1)).unwrap();
            assert_eq!(attraction.lat, 0);
            assert_eq!(attraction.lon, 0);
            assert_eq!(attraction.name, "");
            assert_eq!(attraction.description, "");
        }

        #[ink::test]
        fn set_location_works() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
            contract.set_location(Id::U8(1), 1, 2).unwrap();
            let (lat, lon) = contract.get_location(Id::U8(1)).unwrap();
            assert_eq!(lat, 1);
            assert_eq!(lon, 2);
        }

        #[ink::test]
        fn set_name_works() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
            contract.set_name(Id::U8(1), "test".to_string()).unwrap();
            let name = contract.get_name(Id::U8(1)).unwrap();
            assert_eq!(name, "test");
        }

        #[ink::test]
        fn set_description_works() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
            contract.set_description(Id::U8(1), "test".to_string()).unwrap();
            let description = contract.get_description(Id::U8(1)).unwrap();
            assert_eq!(description, "test");
        }

        #[ink::test]
        fn update_token_works() {
            let mut contract = Contract::new();
            assert_eq!(PSP34Impl::total_supply(&contract), 0);
            contract.mint(address_of!(Alice), Id::U8(1)).unwrap();
            assert_eq!(PSP34Impl::total_supply(&contract), 1);
            contract.update_token(Id::U8(1), "test".to_string(), "test".to_string(), 1, 2).unwrap();
            let (lat, lon) = contract.get_location(Id::U8(1)).unwrap();
            assert_eq!(lat, 1);
            assert_eq!(lon, 2);
            let name = contract.get_name(Id::U8(1)).unwrap();
            assert_eq!(name, "test");
            let description = contract.get_description(Id::U8(1)).unwrap();
            assert_eq!(description, "test");
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod e2e_tests {
        use openbrush::contracts::psp34::extensions::{
            burnable::psp34burnable_external::PSP34Burnable,
            enumerable::psp34enumerable_external::PSP34Enumerable,
            mintable::psp34mintable_external::PSP34Mintable,
        };
        use openbrush::contracts::psp34::psp34_external::PSP34;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use test_helpers::{address_of, balance_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, Alice), 0);
            assert_eq!(balance_of!(client, address, Bob), 0);

            let id_1 = Id::U8(1);
            let id_2 = Id::U8(2);

            let mint_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Alice), id_1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), id_2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_2, Ok(()));

            assert_eq!(balance_of!(client, address, Alice), 1);
            assert_eq!(balance_of!(client, address, Bob), 1);

            Ok(())
        }

        #[ink_e2e::test]
        async fn mint_existing_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, Alice), 0);
            assert_eq!(balance_of!(client, address, Bob), 0);

            let id_1 = Id::U8(1);

            let mint_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Alice), id_1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), id_1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(mint_2, Err(_)));

            assert_eq!(balance_of!(client, address, Alice), 1);
            assert_eq!(balance_of!(client, address, Bob), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let psp34_id1 = Id::U8(1u8);
            let psp34_id2 = Id::U8(2u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Bob), 1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Ok(psp34_id2.clone()));

            let token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(psp34_id2.clone()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works_after_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let psp34_id1 = Id::U8(1u8);
            let psp34_id2 = Id::U8(2u8);

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(psp34_id2.clone()));

            let burn_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(Bob), psp34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(burn_result_1, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(Bob), 1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(psp34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Err(PSP34Error::TokenNotExists));

            Ok(())
        }

        #[ink_e2e::test]
        async fn set_location_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let psp34_id1 = Id::U8(1u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));

            let set_location_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_location(psp34_id1.clone(), 1, 2));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(set_location_result, Ok(()));

            let get_location_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_location(psp34_id1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_location_result, Ok((1, 2)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn set_name_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let psp34_id1 = Id::U8(1u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));

            let set_name_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_name(psp34_id1.clone(), "test".to_string()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(set_name_result, Ok(()));

            let get_name_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_name(psp34_id1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_name_result, Ok("test".to_string()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn set_description_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let psp34_id1 = Id::U8(1u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));

            let set_description_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_description(psp34_id1.clone(), "test".to_string()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(set_description_result, Ok(()));

            let get_description_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_description(psp34_id1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_description_result, Ok("test".to_string()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn update_token_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "flyconomy_attractions",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let psp34_id1 = Id::U8(1u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(Bob), psp34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));

            let update_token_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.update_token(psp34_id1.clone(), "test".to_string(), "test".to_string(), 1, 2));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(update_token_result, Ok(()));

            let get_location_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_location(psp34_id1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_location_result, Ok((1, 2)));

            let get_name_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_name(psp34_id1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_name_result, Ok("test".to_string()));

            let get_description_result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_description(psp34_id1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }.return_value();
            assert_eq!(get_description_result, Ok("test".to_string()));
            Ok(())
        }
    }
}
