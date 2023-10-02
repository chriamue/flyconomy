use async_trait::async_trait;
use serde_json::Value;
use std::str::FromStr;
use web3::contract::Contract;
#[cfg(not(target_arch = "wasm32"))]
use web3::transports::WebSocket;
#[cfg(target_arch = "wasm32")]
use web3::transports::Http;
use web3::types::{Address, U256};

use crate::Attraction;
use crate::AttractionContract;

#[cfg(not(target_arch = "wasm32"))]
pub type Transport = WebSocket;
#[cfg(target_arch = "wasm32")]
pub type Transport = Http;

pub const PRECITION: f64 = 10000.0;

#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_NODE_URL: &str = "wss://sepolia.infura.io/ws/v3/ddb5feac7d6e4ee8b45fdc2ff9355c54";

#[cfg(target_arch = "wasm32")]
pub const DEFAULT_NODE_URL: &str = "https://sepolia.infura.io/v3/ddb5feac7d6e4ee8b45fdc2ff9355c54";

pub const DEFAULT_CONTRACT_ADDRESS: &str = "0x6338b648a9156827e3423A33cb2d32b09076906b";

pub async fn create_contract(
    node_url: &str,
    contract_address: &str,
) -> Result<Contract<Transport>, Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    let transport = Transport::new(&node_url).await?;
    #[cfg(target_arch = "wasm32")]
    let transport = Transport::new(&node_url)?;
    let web3 = web3::Web3::new(transport);
    let contract_address = Address::from_str(&contract_address[2..])?;

    let contract_bytes = include_bytes!(
        "FlyconomyAttractions.json"
    );
    let contract_json: Value = serde_json::from_slice(contract_bytes)?;
    let abi = contract_json["abi"]
        .as_array()
        .ok_or("Failed to extract ABI")?;

    let contract = Contract::from_json(web3.eth(), contract_address, &serde_json::to_vec(abi)?)?;
    Ok(contract)
}

pub struct Web3Contract {
    contract: Contract<Transport>,
}

impl Web3Contract {
    pub async fn new(
        node_url: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let contract = create_contract(node_url, contract_address).await?;
        Ok(Self { contract })
    }
}

#[async_trait(?Send)]
impl AttractionContract for Web3Contract {
    async fn get_all_locations(&self) -> Result<Vec<Attraction>, Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query(
                "getAllLocations",
                (),
                None,
                web3::contract::Options::default(),
                None,
            )
            .await?;
        let (ids, lats, lons): (Vec<U256>, Vec<i32>, Vec<i32>) = result;

        let mut attractions = Vec::new();

        for (i, id) in ids.iter().enumerate() {
            let id = id.as_u64();
            let lat = lats[i];
            let lon = lons[i];
            let name: String = self.get_name(id as u64).await?;
            let description: String = self.get_description(id as u64).await?;

            let attraction = Attraction::new(
                id as u64,
                lat as f64 / PRECITION,
                lon as f64 / PRECITION,
                name,
                description,
            );
            attractions.push(attraction);
        }
        Ok(attractions)
    }

    async fn get_name(&self, id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query(
                "getName",
                id,
                None,
                web3::contract::Options::default(),
                None,
            )
            .await?;
        let name: String = result;
        Ok(name)
    }

    async fn get_description(&self, id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query(
                "getDescription",
                id,
                None,
                web3::contract::Options::default(),
                None,
            )
            .await?;
        let description: String = result;
        Ok(description)
    }

    async fn get_lat(&self, id: u64) -> Result<f64, Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query("getLat", id, None, web3::contract::Options::default(), None)
            .await?;
        let lat: i32 = result;
        Ok(lat as f64 / PRECITION)
    }

    async fn get_lon(&self, id: u64) -> Result<f64, Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query("getLon", id, None, web3::contract::Options::default(), None)
            .await?;
        let lon: i32 = result;
        Ok(lon as f64 / PRECITION)
    }

    async fn get_location(&self, id: u64) -> Result<(f64, f64), Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query(
                "getLocation",
                id,
                None,
                web3::contract::Options::default(),
                None,
            )
            .await?;
        let (lat, lon): (i32, i32) = result;
        Ok((lat as f64 / PRECITION, lon as f64 / PRECITION))
    }

    async fn get_total_supply(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let result = self
            .contract
            .query(
                "totalSupply",
                (),
                None,
                web3::contract::Options::default(),
                None,
            )
            .await?;
        let total_supply: u64 = result;
        Ok(total_supply)
    }
}
