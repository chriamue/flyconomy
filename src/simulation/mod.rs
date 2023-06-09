use std::time::Duration;

use crate::model::CompanyFinances;

pub struct Simulation {
    pub company_finances: CompanyFinances,
    elapsed_time: Duration,
}

impl Simulation {
    pub fn new(capital: f64) -> Self {
        Self {
            company_finances: CompanyFinances::new(capital),
            elapsed_time: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.elapsed_time += delta_time;
        self.company_finances.cash = self.company_finances.cash - 1.0;
    }
}
