use super::Timestamp;

#[derive(Debug, Default)]
pub struct CompanyFinances {
    pub income: Vec<(Timestamp, f64)>,
    pub expenses: Vec<(Timestamp, f64)>,
}

impl CompanyFinances {
    pub fn new(cash: f64) -> Self {
        Self {
            income: vec![(0, cash)],
            expenses: vec![],
        }
    }

    pub fn add_income(&mut self, timestamp: Timestamp, income: f64) {
        self.income.push((timestamp, income));
    }

    pub fn add_expense(&mut self, timestamp: Timestamp, expense: f64) {
        self.expenses.push((timestamp, expense));
    }

    pub fn cash(&self, timestamp: Timestamp) -> f64 {
        let mut cash = 0.0;
        for (income_timestamp, income) in &self.income {
            if *income_timestamp <= timestamp {
                cash += income;
            }
        }
        for (expense_timestamp, expense) in &self.expenses {
            if *expense_timestamp <= timestamp {
                cash -= expense;
            }
        }
        cash
    }

    pub fn total_income(&self, timestamp: Timestamp) -> f64 {
        let mut total_income = 0.0;
        for (income_timestamp, income) in &self.income {
            if *income_timestamp <= timestamp {
                total_income += income;
            }
        }
        total_income
    }

    pub fn total_expenses(&self, timestamp: Timestamp) -> f64 {
        let mut total_expenses = 0.0;
        for (expense_timestamp, expense) in &self.expenses {
            if *expense_timestamp <= timestamp {
                total_expenses += expense;
            }
        }
        total_expenses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_company_finances() {
        let finances = CompanyFinances::new(100.0);
        assert_eq!(finances.cash(0), 100.0);
    }

    #[test]
    fn test_add_income() {
        let mut finances = CompanyFinances::new(100.0);
        finances.add_income(1, 50.0);
        assert_eq!(finances.total_income(1), 150.0);
        assert_eq!(finances.total_income(0), 100.0); // Timestamp is 0, should be initial income
    }

    #[test]
    fn test_add_expense() {
        let mut finances = CompanyFinances::new(100.0);
        finances.add_expense(1, 50.0);
        assert_eq!(finances.total_expenses(1), 50.0);
        assert_eq!(finances.total_expenses(0), 0.0); // Timestamp is 0, should be no expenses
    }

    #[test]
    fn test_cash() {
        let mut finances = CompanyFinances::new(100.0);
        finances.add_income(1, 50.0);
        finances.add_expense(2, 30.0);
        assert_eq!(finances.cash(3), 120.0); // 100 initial + 50 income - 30 expenses
    }
}
