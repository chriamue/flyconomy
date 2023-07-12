use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use flyconomy::model::analytics::{calculate_cash_history, calculate_total_flight_distance};
use flyconomy::model::{CompanyFinances, Environment, Flight, FlightState};

const TIMESTAMP_COLLECTION: [u128; 3] = [1000, 10_000, 100_000];

fn bench_calculate_cash_history(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_cash_history");

    for timestamp in TIMESTAMP_COLLECTION.iter() {
        let company_finances = CompanyFinances::new(0.0);
        let mut environment = Environment {
            timestamp: 0,
            company_finances,
            ..Default::default()
        };

        for _ in 0..*timestamp {
            environment.timestamp += 1;
            environment
                .company_finances
                .add_income(environment.timestamp, 1.0);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(timestamp),
            timestamp,
            |b, _t| b.iter(|| calculate_cash_history(&environment)),
        );
    }

    group.finish();
}

fn bench_calculate_total_flight_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_total_flight_distance");

    for timestamp in TIMESTAMP_COLLECTION.iter() {
        let mut environment = Environment::default();
        environment.timestamp = *timestamp;

        environment.flights = (0..=*timestamp)
            .step_by(100)
            .map(|i| Flight {
                state: FlightState::Finished,
                arrival_time: Some(i as u128),
                ..Default::default()
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(timestamp),
            timestamp,
            |b, _t| b.iter(|| calculate_total_flight_distance(&environment)),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_calculate_cash_history,
    bench_calculate_total_flight_distance
);

criterion_main!(benches);
