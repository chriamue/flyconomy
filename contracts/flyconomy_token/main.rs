use flyconomy_token::{FlyconomyTokenRef};
use ink::{primitives::AccountId};
use ink_e2e::subxt::utils::AccountId32;
use ink_e2e::{
    build_message, subxt::config::SubstrateConfig, subxt::OnlineClient, Client,
};
use ink_env::DefaultEnvironment;
use openbrush::contracts::psp22::psp22_external::PSP22;
use std::str::FromStr;

fn address_from_string(address: String) -> AccountId {
    let address: AccountId32 = AccountId32::from_str(&address).unwrap();
    address.0.into()
}

#[tokio::main]
async fn main() {
    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();

    println!("Connected to node: {:?}", client.runtime_version());

    let mut api =
        Client::<SubstrateConfig, DefaultEnvironment>::new(client, []).await;

    let alice_account_id =
        address_from_string("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string());

    let contract_address: AccountId =
        address_from_string("5HKJ1GtqcvfxyMiK41KjgMb5rjiaT1uyQUHCqbC9KBqyLmWG".to_string());

    println!("Contract Address {:?}", contract_address);

    let _msg = build_message::<FlyconomyTokenRef>(contract_address.clone())
        .call(|contract| contract.balance_of(alice_account_id));

    let result = api
        .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
        .await
        .return_value();

    println!("Balance of Alice {:?}", result);

    let bob_account_id =
        address_from_string("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string());

    let _msg = build_message::<FlyconomyTokenRef>(contract_address.clone())
        .call(|contract| contract.balance_of(bob_account_id));

    let result = api
        .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
        .await
        .return_value();

    println!("Balance of Bob {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink_env::test::DefaultAccounts;

    #[test]
    fn new_works() {
        let accounts: DefaultAccounts<DefaultEnvironment> = ink_env::test::default_accounts();

        println!("{:?}", accounts.alice);
    }
}
