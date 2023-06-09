use crate::model::{CompanyFinances, PlaneType};

pub struct Environment {
    pub company_finances: CompanyFinances,
    pub planes: Vec<PlaneType>,
}
impl Environment {
    pub fn new(capital: f64) -> Self {
        Self {
            company_finances: CompanyFinances::new(capital),
            planes: vec![],
        }
    }
}
