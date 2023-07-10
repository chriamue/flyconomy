use serde::{Deserialize, Serialize};

use crate::model::{Aerodrome, EnvironmentConfig, PlaneType, WorldHeritageSite};

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

pub fn plane_types() -> Vec<PlaneType> {
    let yaml_content = include_str!("../../assets/planes.yaml");
    let planes: PlanesConfig = serde_yaml::from_str(yaml_content).unwrap();
    planes.planes
}

pub fn aerodromes() -> Vec<Aerodrome> {
    load_airports(
        include_str!("../../assets/airports.dat"),
        include_str!("../../assets/passengers.csv"),
    )
}

pub fn world_heritage_sites() -> Vec<WorldHeritageSite> {
    parse_world_heritage_site_csv(include_str!("../../assets/whc-sites-2019.csv"))
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
