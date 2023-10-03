use flyconomy_contracts_client::{
    Attraction, AttractionContract, Web3Contract, DEFAULT_CONTRACT_ADDRESS,
};

pub async fn update(attraction: Attraction) -> Result<(), Box<dyn std::error::Error>> {
    let contract = Web3Contract::new_eip1193(DEFAULT_CONTRACT_ADDRESS).await?;

    contract.update(attraction).await?;
    Ok(())
}
