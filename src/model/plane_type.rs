use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaneType {
    pub id: u32,
    pub name: String,
    pub cost: f32,
    pub monthly_income: f32,
    pub speed: f32,
    pub range: f32,
    pub seats: u32,
    pub fuel_consumption_per_km: f32,
}

impl Default for PlaneType {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Small Plane"),
            cost: 100000.0,
            monthly_income: 0.0,
            speed: 800.0,
            range: 4000.0,
            seats: 150,
            fuel_consumption_per_km: 3.0,
        }
    }
}
