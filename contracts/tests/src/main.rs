use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::str::FromStr;
use structopt::StructOpt;
use web3::contract::Contract;
use web3::transports::Http;
use web3::types::{Address, U256};

#[derive(StructOpt, Debug)]
#[structopt(name = "contract-cli")]
pub struct Opt {
    /// Function to call
    #[structopt(subcommand)]
    pub cmd: Command,
}

/// Commands for interacting with the FlyconomyAttractions contract
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Query the total supply of tokens
    TotalSupply,
    /// Query the location of a token by ID
    Location {
        /// The ID of the token
        id: u64,
    },
    /// Query the name of a token by ID
    Name {
        /// The ID of the token
        id: u64,
    },
    /// Query the description of a token by ID
    Description {
        /// The ID of the token
        id: u64,
    },

    /// Query all locations
    AllLocations,
}

async fn create_contract(
    node_url: String,
    contract_address: String,
) -> Result<Contract<Http>, Box<dyn std::error::Error>> {
    let http = Http::new(&node_url)?;
    let web3 = web3::Web3::new(http);
    let contract_address = Address::from_str(&contract_address[2..])?;

    let contract_bytes = include_bytes!(
        "../../artifacts/contracts/FlyconomyAttractions.sol/FlyconomyAttractions.json"
    );
    let contract_json: Value = serde_json::from_slice(contract_bytes)?;
    let abi = contract_json["abi"]
        .as_array()
        .ok_or("Failed to extract ABI")?;

    let contract = Contract::from_json(web3.eth(), contract_address, &serde_json::to_vec(abi)?)?;
    Ok(contract)
}

#[derive(Debug, Deserialize)]
struct AllLocations {
    ids: Vec<U256>,
    lats: Vec<i32>,
    lons: Vec<i32>,
}

async fn get_all_locations(
    contract: &Contract<Http>,
) -> Result<AllLocations, Box<dyn std::error::Error>> {
    let result = contract
        .query(
            "getAllLocations",
            (),
            None,
            web3::contract::Options::default(),
            None,
        )
        .await?;
    let (ids, lats, lons): (Vec<U256>, Vec<i32>, Vec<i32>) = result;
    Ok(AllLocations { ids, lats, lons })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let node_url = env::var("NODE_URL").expect("NODE_URL is not set in .env file");
    let contract_address = env::var("CONTRACT").expect("CONTRACT is not set in .env file");

    let opt = Opt::from_args();
    let contract = create_contract(node_url, contract_address).await?;

    match opt.cmd {
        Command::TotalSupply => {
            let result = contract
                .query(
                    "totalSupply",
                    (),
                    None,
                    web3::contract::Options::default(),
                    None,
                )
                .await?;
            let total_supply: u64 = result;
            println!("Total Supply: {}", total_supply);
        }
        Command::Location { id } => {
            let result = contract
                .query(
                    "getLocation",
                    id,
                    None,
                    web3::contract::Options::default(),
                    None,
                )
                .await?;
            let (lat, lon): (i32, i32) = result;
            println!(
                "Location of ID {}: Latitude: {}, Longitude: {}",
                id, lat, lon
            );
        }
        Command::Name { id } => {
            let result = contract
                .query(
                    "getName",
                    id,
                    None,
                    web3::contract::Options::default(),
                    None,
                )
                .await?;
            let name: String = result;
            println!("Name of ID {}: {}", id, name);
        }
        Command::Description { id } => {
            let result = contract
                .query(
                    "getDescription",
                    id,
                    None,
                    web3::contract::Options::default(),
                    None,
                )
                .await?;
            let description: String = result;
            println!("Description of ID {}: {}", id, description);
        }
        Command::AllLocations => {
            let locations = get_all_locations(&contract).await?;
            for (i, id) in locations.ids.iter().enumerate() {
                println!(
                    "ID: {}, Latitude: {}, Longitude: {}",
                    id, locations.lats[i], locations.lons[i]
                );
            }
        }
    }

    Ok(())
}
