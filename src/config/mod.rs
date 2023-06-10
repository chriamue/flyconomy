use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::PlaneType;

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid, Clone)]
#[uuid = "0f597d24-3263-4083-ace3-f971d2a820b7"]
pub struct PlanesConfig {
    pub planes: Vec<PlaneType>,
}

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid, Clone)]
#[uuid = "45d4e0f3-c25e-4b19-bfb4-ff278fbad7b0"]
pub struct AerodromeConfig(pub Value);

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
        assert_eq!(planes.planes[0].monthly_income, 5000.0);
        assert_eq!(planes.planes[0].monthly_operating_cost, 1000.0);
    }
}
