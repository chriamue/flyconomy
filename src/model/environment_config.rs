use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentConfig {
    pub start_capital: f64,
    pub landing_rights_cost: f64,
    pub base_cost: f64,
    pub takeoff_cost: f64,
    pub fuel_cost_per_km: f64,
    pub income_per_km: f64,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            start_capital: 1_000_000.0,
            landing_rights_cost: 100_000.0,
            base_cost: 400_000.0,
            takeoff_cost: 500.0,
            fuel_cost_per_km: 0.5,
            income_per_km: 5.0,
        }
    }
}
