use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Aerodrome {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}
