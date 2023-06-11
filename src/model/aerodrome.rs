use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aerodrome {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}
