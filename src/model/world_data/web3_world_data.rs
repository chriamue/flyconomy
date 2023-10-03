use super::*;
use crate::model::Attraction;
use async_std::task::block_on;
use flyconomy_contracts_client::{web3::*, AttractionContract, Web3Contract};
use std::sync::Arc;
use std::sync::RwLock;

pub struct Web3WorldData {
    attractions: Arc<RwLock<Vec<Attraction>>>,
    string_based_world_data: StringBasedWorldData,
}

impl Default for Web3WorldData {
    fn default() -> Self {
        let mut data = Self {
            attractions: Arc::new(RwLock::new(Vec::new())),
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
        let attractions_rwlock = Arc::clone(&self.attractions);

        block_on(async move {
            let result: Result<(), Box<dyn std::error::Error>> = async {
                let contract = {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        Web3Contract::new_websocket(DEFAULT_NODE_URL, DEFAULT_CONTRACT_ADDRESS).await?
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        Web3Contract::new_http(DEFAULT_NODE_URL, DEFAULT_CONTRACT_ADDRESS).await?
                    }
                };
                let attractions = contract.get_all_locations().await?;

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

                log::info!("Got attractions: {:?}", attractions);
                let mut attractions_lock = attractions_rwlock.write().unwrap();
                *attractions_lock = attractions;
                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Failed to get attractions: {}", e);
            }
        });

        Ok(())
    }
}

impl WorldDataGateway for Web3WorldData {
    fn aerodromes(&self) -> &Vec<Aerodrome> {
        self.string_based_world_data.aerodromes()
    }

    fn attractions(&self) -> Vec<Attraction> {
        self.attractions.read().unwrap().to_vec()
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
