use crate::model::CompanyFinances;

use super::{AirPlane, Base, Flight, LandingRights};

pub struct Environment {
    pub company_finances: CompanyFinances,
    pub planes: Vec<AirPlane>,
    pub bases: Vec<Base>,
    pub landing_rights: Vec<LandingRights>,
    pub flights: Vec<Flight>,
}
impl Environment {
    pub fn new(capital: f64) -> Self {
        Self {
            company_finances: CompanyFinances::new(capital),
            planes: vec![],
            bases: vec![],
            landing_rights: vec![],
            flights: vec![],
        }
    }
}
