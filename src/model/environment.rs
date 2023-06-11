use crate::model::{CompanyFinances};

use super::{Base, AirPlane};

pub struct Environment {
    pub company_finances: CompanyFinances,
    pub planes: Vec<AirPlane>,
    pub bases: Vec<Base>,
}
impl Environment {
    pub fn new(capital: f64) -> Self {
        Self {
            company_finances: CompanyFinances::new(capital),
            planes: vec![],
            bases: vec![],
        }
    }
}
