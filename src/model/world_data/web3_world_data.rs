use super::*;
use crate::model::Attraction;
use async_std::task;
use flyconomy_contracts_client::{web3::*, AttractionContract, Web3Contract};
use std::sync::Arc;

pub struct Web3WorldData {
    attractions: Arc<Vec<Attraction>>,
    string_based_world_data: StringBasedWorldData,
}

impl Default for Web3WorldData {
    fn default() -> Self {
        let mut data = Self {
            attractions: Arc::new(Vec::new()),
            string_based_world_data: StringBasedWorldData::default(),
        };
        match data.reload() {
            Ok(_) => {
                log::info!("Loaded world data");
            }
            Err(e) => {
                log::error!("Failed to load world data: {}", e);
            }
        }
        data
    }
}

impl Web3WorldData {
    pub fn reload_attractions(&mut self) -> Result<(), String> {
        let attractions: Vec<flyconomy_contracts_client::Attraction> = task::block_on(async move {
            let contract = Web3Contract::new(DEFAULT_NODE_URL, DEFAULT_CONTRACT_ADDRESS).await?;
            contract.get_all_locations().await
        })
        .map_err(|e| format!("Failed to get attractions: {}", e))?;

        let attractions: Vec<Attraction> = attractions
            .into_iter()
            .map(|attraction| Attraction {
                id: attraction.id,
                name: attraction.name,
                description: attraction.description,
                lat: attraction.lat,
                lon: attraction.lon,
            })
            .collect();

        self.attractions = Arc::new(attractions);
        Ok(())
    }
}

impl WorldDataGateway for Web3WorldData {
    fn aerodromes(&self) -> &Vec<Aerodrome> {
        self.string_based_world_data.aerodromes()
    }

    fn attractions(&self) -> &Vec<Attraction> {
        &self.attractions
    }

    fn world_heritage_sites(&self) -> &Vec<WorldHeritageSite> {
        self.string_based_world_data.world_heritage_sites()
    }

    fn plane_types(&self) -> &Vec<PlaneType> {
        self.string_based_world_data.plane_types()
    }

    fn reload(&mut self) -> Result<(), String> {
        self.string_based_world_data.reload()?;
        self.reload_attractions()?;
        Ok(())
    }
}
