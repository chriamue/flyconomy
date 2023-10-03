use async_trait::async_trait;
use serde_json::Value;
use web3::Transport;
use web3::Web3;
use std::str::FromStr;
use web3::contract::Contract;
#[cfg(not(target_arch = "wasm32"))]
use web3::transports::WebSocket;
#[cfg(target_arch = "wasm32")]
use web3::transports::Http;
#[cfg(target_arch = "wasm32")]
use web3::transports::eip_1193::Eip1193;
use web3::types::{Address, U256};

use crate::Attraction;
use crate::AttractionContract;

pub const PRECITION: f64 = 10000.0;

#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_NODE_URL: &str = "wss://sepolia.infura.io/ws/v3/ddb5feac7d6e4ee8b45fdc2ff9355c54";

#[cfg(target_arch = "wasm32")]
pub const DEFAULT_NODE_URL: &str = "https://sepolia.infura.io/v3/ddb5feac7d6e4ee8b45fdc2ff9355c54";

pub const DEFAULT_CONTRACT_ADDRESS: &str = "0x6338b648a9156827e3423A33cb2d32b09076906b";

pub async fn create_contract<T: Transport>(
    contract_address: &str,
    transport: T,
) -> Result<(Web3<T>, Contract<T>), Box<dyn std::error::Error>> {
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
    Ok((web3, contract))
}

pub struct Web3Contract<T> where T: Transport {
    web3: Web3<T>,
    contract: Contract<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Web3Contract<WebSocket> {
    pub async fn new_websocket(
        node_url: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (web3, contract) = create_contract(contract_address, WebSocket::new(node_url).await?).await?;
        Ok(Self { web3, contract })
    }
}

#[cfg(target_arch = "wasm32")]
impl Web3Contract<Http> {
    pub async fn new_http(
        node_url: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (web3, contract) = create_contract(contract_address, Http::new(node_url)?).await?;
        Ok(Self { web3, contract })
    }
}

#[cfg(target_arch = "wasm32")]
impl Web3Contract<Eip1193> {
    pub async fn new_eip1193(
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = web3::transports::eip_1193::Provider::default().unwrap().unwrap();
        let transport =  web3::transports::eip_1193::Eip1193::new(provider);
        let (web3, contract) = create_contract(contract_address, transport).await?;
        Ok(Self { web3, contract })
    }
}

#[async_trait(?Send)]
impl <T: Transport> AttractionContract for Web3Contract<T> {
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

    async fn update(
        &self, attraction: Attraction
    ) -> Result<(), Box<dyn std::error::Error>> {
        let attraction_id = attraction.id;
        let attraction_name = attraction.name;
        let attraction_description = attraction.description;
        let attraction_lat = (attraction.lat * PRECITION) as i32;
        let attraction_lon = (attraction.lon * PRECITION) as i32;

        let addrs = self.web3.eth().request_accounts().await?;
        let from = addrs[0];

        let _result = self
            .contract
            .call(
                "updateToken",
                (
                    attraction_id,
                    attraction_name,
                    attraction_description,
                    attraction_lat,
                    attraction_lon,
                ),
                from,
                web3::contract::Options::default(),
            )
            .await?;
        Ok(())
    }

    async fn mint(
        &self, attraction: Attraction
    ) -> Result<(), Box<dyn std::error::Error>> {
        let attraction_name = attraction.name;
        let attraction_description = attraction.description;
        let attraction_lat = (attraction.lat * PRECITION) as i32;
        let attraction_lon = (attraction.lon * PRECITION) as i32;

        let addrs = self.web3.eth().request_accounts().await?;
        let from = addrs[0];

        let _result = self
            .contract
            .call(
                "mint",
                (
                    from,
                ),
                from,
                web3::contract::Options::default(),
            )
            .await?;
        Ok(())
    }
}
