#[derive(Debug, Default)]
pub struct CompanyFinances {
    pub cash: f64,
    pub total_income: f32,
    pub total_expenses: f32,
}

impl CompanyFinances {
    pub fn new(cash: f64) -> Self {
        Self {
            cash,
            total_income: 0.0,
            total_expenses: 0.0,
        }
    }
}
