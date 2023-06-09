use serde::{Deserialize, Serialize};

use crate::model::PlaneType;

#[derive(Serialize, Deserialize, Debug, bevy::reflect::TypeUuid)]
#[uuid = "0f597d24-3263-4083-ace3-f971d2a820b7"]
pub struct PlanesConfig {
    pub planes: Vec<PlaneType>,
}
