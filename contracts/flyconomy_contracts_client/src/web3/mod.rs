use serde_json::Value;
use std::str::FromStr;
use web3::contract::Contract;
use web3::transports::Http;
use web3::types::Address;

pub async fn create_contract(
    node_url: String,
    contract_address: String,
) -> Result<Contract<Http>, Box<dyn std::error::Error>> {
    let http = Http::new(&node_url)?;
    let web3 = web3::Web3::new(http);
    let contract_address = Address::from_str(&contract_address[2..])?;

    let contract_bytes = include_bytes!(
        "../../../artifacts/contracts/FlyconomyAttractions.sol/FlyconomyAttractions.json"
    );
    let contract_json: Value = serde_json::from_slice(contract_bytes)?;
    let abi = contract_json["abi"]
        .as_array()
        .ok_or("Failed to extract ABI")?;

    let contract = Contract::from_json(web3.eth(), contract_address, &serde_json::to_vec(abi)?)?;
    Ok(contract)
}
