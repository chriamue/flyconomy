use serde::{Deserialize, Serialize};

use super::PlaneType;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AirPlane {
    pub id: u64,
    pub base_id: u64,
    pub plane_type: PlaneType,
}
