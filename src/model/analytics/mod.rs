use super::{Environment, Timestamp};

pub fn calculate_cash_history(environment: &Environment) -> Vec<(Timestamp, f64)> {
    let mut cash_history = vec![];

    let total_timestamps = environment.timestamp;
    let sample_interval = (total_timestamps / 100).max(1);

    for timestamp in (0..total_timestamps + sample_interval).step_by(sample_interval as usize) {
        let cash = environment.company_finances.cash(timestamp);
        cash_history.push((timestamp, cash));
    }

    cash_history
}
