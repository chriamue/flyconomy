use serde::{Deserialize, Serialize};

use crate::model::{EnvironmentConfig, PlaneType};

mod aerodrome_config;
mod world_heritage_site_config;

pub use aerodrome_config::{load_airports, AerodromeConfig};
pub use world_heritage_site_config::parse_world_heritage_site_csv;

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid, Clone)]
#[uuid = "0f597d24-3263-4083-ace3-f971d2a820b7"]
pub struct PlanesConfig {
    pub planes: Vec<PlaneType>,
}

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid, Clone)]
#[uuid = "0f597d24-3263-4083-ace3-f971d2a820b8"]
pub struct LevelConfig {
    pub name: String,
    pub description: String,
    pub environment: EnvironmentConfig,
}

impl Default for LevelConfig {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            description: String::from("Default level"),
            environment: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_planes() {
        let yaml_content = include_str!("../../assets/planes.yaml");

        let planes: PlanesConfig = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(planes.planes.len(), 3);
        assert_eq!(planes.planes[0].name, "Small Plane");
        assert_eq!(planes.planes[0].cost, 300000.0);
        assert_eq!(planes.planes[0].monthly_income, 0.0);
    }
}
