mod environment_iterators;
use crate::model::CompanyFinances;

use super::{AirPlane, Base, EnvironmentConfig, Flight, LandingRights, Timestamp};

#[derive(Debug, Clone)]
pub struct Environment {
    pub config: EnvironmentConfig,
    pub company_finances: CompanyFinances,
    pub planes: Vec<AirPlane>,
    pub bases: Vec<Base>,
    pub landing_rights: Vec<LandingRights>,
    pub flights: Vec<Flight>,
    pub timestamp: Timestamp,
    pub last_errors: Vec<(Timestamp, String)>,
}
impl Environment {
    pub fn new(config: EnvironmentConfig) -> Self {
        Self {
            company_finances: CompanyFinances::new(config.start_capital),
            config,
            planes: vec![],
            bases: vec![],
            landing_rights: vec![],
            flights: vec![],
            timestamp: 0,
            last_errors: vec![],
        }
    }

    pub fn get_errors_indicator(&self) -> u64 {
        let mut indicator = 0.0;
        for error in &self.last_errors {
            let time_since_error = self.timestamp - error.0;
            indicator += 1.0 / (time_since_error as f64 + 1.0);
        }
        (indicator * 1_000_000.0).round() as u64
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(EnvironmentConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_errors_indicator() {
        let mut environment = Environment::new(EnvironmentConfig::default());

        // No errors, should return 0
        assert_eq!(environment.get_errors_indicator(), 0);

        // Add one error
        environment.last_errors.push((0, "Error 1".to_string()));
        environment.timestamp = 1;
        assert_eq!(environment.get_errors_indicator(), 500_000);

        // Add another error
        environment.last_errors.push((1, "Error 2".to_string()));
        environment.timestamp = 2;
        let expected_indicator = 833333;
        assert_eq!(environment.get_errors_indicator(), expected_indicator);

        // Increase timestamp without adding error
        environment.timestamp = 10;
        let expected_indicator = 190909;
        assert_eq!(environment.get_errors_indicator(), expected_indicator);

        // Increase timestamp even further
        environment.timestamp = 100;
        let expected_indicator = 19901;
        assert_eq!(environment.get_errors_indicator(), expected_indicator);

        // Add a third error
        environment.last_errors.push((100, "Error 3".to_string()));
        environment.timestamp = 101;
        let expected_indicator = 519705;
        assert_eq!(environment.get_errors_indicator(), expected_indicator);
    }
}
