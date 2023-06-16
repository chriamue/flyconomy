use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EnvironmentConfig {
    pub start_capital: f64,
    pub landing_rights_cost: f64,
    pub base_cost: f64,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            start_capital: 1_000_000.0,
            landing_rights_cost: 100_000.0,
            base_cost: 400_000.0,
        }
    }
}
