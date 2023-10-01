use flyconomy_contracts_client::{AttractionContract, Web3Contract};
use std::env;
use structopt::StructOpt;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let node_url = env::var("NODE_URL").expect("NODE_URL is not set in .env file");
    let contract_address = env::var("CONTRACT").expect("CONTRACT is not set in .env file");

    let opt = Opt::from_args();

    let contract: Box<dyn AttractionContract> =
        Box::new(Web3Contract::new(node_url, contract_address).await?);

    match opt.cmd {
        Command::TotalSupply => {
            let total_supply: u64 = contract.get_total_supply().await?;
            println!("Total Supply: {}", total_supply);
        }
        Command::Location { id } => {
            let (lat, lon): (f64, f64) = contract.get_location(id).await?;
            println!(
                "Location of ID {}: Latitude: {}, Longitude: {}",
                id, lat, lon
            );
        }
        Command::Name { id } => {
            let name: String = contract.get_name(id).await?;
            println!("Name of ID {}: {}", id, name);
        }
        Command::Description { id } => {
            let description: String = contract.get_description(id).await?;
            println!("Description of ID {}: {}", id, description);
        }
        Command::AllLocations => {
            let locations = contract.get_all_locations().await?;
            for (i, attraction) in locations.iter().enumerate() {
                println!("ID: {}, {:?}", i, attraction);
            }
        }
    }

    Ok(())
}
