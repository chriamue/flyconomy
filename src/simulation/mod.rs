use std::{time::Duration, vec};

use crate::model::{CompanyFinances, PlaneType};

pub struct Simulation {
    pub company_finances: CompanyFinances,
    pub plane_types: Vec<PlaneType>,
    pub planes: Vec<PlaneType>,
    elapsed_time: Duration,
}

impl Simulation {
    pub fn new(capital: f64) -> Self {
        Self {
            company_finances: CompanyFinances::new(capital),
            plane_types: vec![],
            elapsed_time: Duration::from_secs(0),
            planes: vec![],
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.elapsed_time += delta_time;
        self.company_finances.cash = self.company_finances.cash - 1.0;
    }
}
