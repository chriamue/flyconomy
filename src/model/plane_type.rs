use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaneType {
    pub name: String,
    pub cost: f32,
    pub monthly_income: f32,
    pub monthly_operating_cost: f32,
}