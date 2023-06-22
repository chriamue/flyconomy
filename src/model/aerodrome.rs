use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Aerodrome {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}

impl Aerodrome {
    pub fn new(id: u64, lat: f64, lon: f64, name: String) -> Self {
        Self { id, lat, lon, name }
    }
}
