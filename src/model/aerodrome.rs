use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Aerodrome {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}
