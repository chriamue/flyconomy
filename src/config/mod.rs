use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::PlaneType;

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid)]
#[uuid = "0f597d24-3263-4083-ace3-f971d2a820b7"]
pub struct PlanesConfig {
    pub planes: Vec<PlaneType>,
}

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid)]
#[uuid = "45d4e0f3-c25e-4b19-bfb4-ff278fbad7b0"]
pub struct AerodromeConfig(pub Value);
