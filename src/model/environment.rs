use crate::model::CompanyFinances;

use super::{AirPlane, Base, EnvironmentConfig, Flight, LandingRights};

pub struct Environment {
    pub config: EnvironmentConfig,
    pub company_finances: CompanyFinances,
    pub planes: Vec<AirPlane>,
    pub bases: Vec<Base>,
    pub landing_rights: Vec<LandingRights>,
    pub flights: Vec<Flight>,
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
        }
    }
}
