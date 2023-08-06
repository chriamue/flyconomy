use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[derive(Clone)]
pub struct TokenService {
    client: OnlineClient<PolkadotConfig>,
}

impl TokenService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = OnlineClient::<PolkadotConfig>::new().await?;
        let contract_address_str = "5HKJ1GtqcvfxyMiK41KjgMb5rjiaT1uyQUHCqbC9KBqyLmWG".to_string();

        Ok(TokenService { client })
    }

    pub async fn get_balance_of(
        &self,
        account_id: &AccountId32,
        account_address: String,
    ) -> Result<u128, Box<dyn std::error::Error>> {
        /*
                // Construct the message for the contract call
                let msg = build_message::<FlyconomyTokenRef>(self.contract_address.clone())
                    .call(|contract| contract.balance_of(account_id));

                let result = self.client
                    .call_dry_run(&sender_id, &msg, 0, None)
                    .await
                    .return_value();
        */
        let result = 42;

        // Assuming result can be converted into u128
        Ok(result)
    }
}
