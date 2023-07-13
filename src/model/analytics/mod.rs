use super::{Environment, FlightState, Timestamp};

#[cfg(feature = "rayon")]
use rayon::iter::ParallelIterator;

const SAMPLES: u128 = 100;

fn timestamp_samples(total_timestamps: u128, samples: u128) -> Box<dyn Iterator<Item = u128>> {
    let sample_interval = (total_timestamps / samples).max(1);
    Box::new(
        (sample_interval..total_timestamps + sample_interval).step_by(sample_interval as usize),
    )
}

pub fn calculate_cash_history(environment: &Environment) -> Vec<(Timestamp, f64)> {
    let mut cash_history = vec![];

    for timestamp in timestamp_samples(environment.timestamp, SAMPLES) {
        let cash = environment.company_finances.cash(timestamp);
        cash_history.push((timestamp, cash));
    }

    cash_history
}

pub fn calculate_total_flight_distance(environment: &Environment) -> Vec<(Timestamp, f64)> {
    let mut flight_distance_history = vec![];

    for timestamp in timestamp_samples(environment.timestamp, SAMPLES) {
        let total_distance = environment
            .iter_flights()
            .filter(|flight| {
                flight.state == FlightState::Finished && flight.arrival_time.unwrap() <= timestamp
            })
            .map(|flight| flight.calculate_total_distance())
            .sum();
        flight_distance_history.push((timestamp, total_distance));
    }

    flight_distance_history
}

pub fn calculate_transported_passengers(environment: &Environment) -> Vec<(Timestamp, u32)> {
    let mut transported_passengers_history = vec![];

    for timestamp in timestamp_samples(environment.timestamp, SAMPLES) {
        let total_passengers = environment
            .iter_flights()
            .filter(|flight| {
                flight.state == FlightState::Finished && flight.arrival_time.unwrap() <= timestamp
            })
            .map(|flight| flight.calculate_booked_seats())
            .sum();
        transported_passengers_history.push((timestamp, total_passengers));
    }

    transported_passengers_history
}

pub fn calculate_average_profit_per_flight(environment: &Environment) -> Vec<(Timestamp, f64)> {
    let mut average_profit_history = vec![];

    let seven_days_in_timestamps = 7 * 24 * 60 * 60 * 1000;

    for timestamp in timestamp_samples(environment.timestamp, SAMPLES) {
        let flights_in_last_seven_days = environment.iter_flights().filter(|flight| {
            flight.state == FlightState::Finished
                && flight.arrival_time.unwrap() <= timestamp
                && flight.arrival_time.unwrap() > timestamp - seven_days_in_timestamps
        });

        let flight_count = flights_in_last_seven_days.clone().count();

        let total_profit: f64 = flights_in_last_seven_days
            .map(|flight| flight.calculate_profit())
            .sum();

        let average_profit = if flight_count > 0 {
            total_profit / flight_count as f64
        } else {
            0.0
        };

        average_profit_history.push((timestamp, average_profit));
    }

    average_profit_history
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CompanyFinances, Flight};

    #[test]
    fn test_calculate_cash_history() {
        let company_finances = CompanyFinances::new(0.0);

        let mut environment = Environment {
            timestamp: 0,
            company_finances,
            ..Default::default()
        };

        for _ in 0..1000 {
            environment.timestamp += 1;
            environment
                .company_finances
                .add_income(environment.timestamp, 1.0);
        }

        let cash_history = calculate_cash_history(&environment);

        assert_eq!(cash_history.len(), SAMPLES as usize);
        for (timestamp, _cash) in &cash_history {
            assert!(*timestamp <= environment.timestamp);
        }

        assert_eq!(cash_history[0].0, 10);
        assert_eq!(
            cash_history[cash_history.len() - 1].0,
            environment.timestamp
        );
        assert_eq!(cash_history[0].1, 10.0);
        assert_eq!(cash_history[cash_history.len() - 1].1, 1000.0);
    }

    #[test]
    fn test_total_flight_distance() {
        let mut environment = Environment::default();
        environment.timestamp = 2000;

        environment.flights = (1..=20)
            .map(|i| Flight {
                state: FlightState::Finished,
                arrival_time: Some(i as u128 * 100),
                ..Default::default()
            })
            .collect();

        let flight_distance_history = calculate_total_flight_distance(&environment);

        assert_eq!(flight_distance_history.len(), SAMPLES as usize);
        for (timestamp, _distance) in &flight_distance_history {
            assert!(*timestamp <= environment.timestamp);
        }

        assert_eq!(flight_distance_history[0].0, 20);
        assert_eq!(
            flight_distance_history.last().unwrap().0,
            environment.timestamp
        );
        assert_eq!(flight_distance_history.first().unwrap().1, 0.0);
        assert!((flight_distance_history.last().unwrap().1 - 18_000.0).abs() < 20.0);
    }
}
