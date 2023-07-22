use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use flyconomy::{
    algorithms::calculate_interest_score,
    model::{StringBasedWorldData, WorldDataGateway},
};

fn bench_calculate_interest_score(c: &mut Criterion) {
    let lat = 40.748817;
    let lon = -73.985428;
    let max_distance = 250_000.0; // meters

    let world_data = StringBasedWorldData::default();

    let heritage_sites: Vec<(f64, f64, f64)> = world_data
        .world_heritage_sites()
        .iter()
        .map(|site| (site.lat, site.lon, 1.0f64))
        .collect();

    c.bench_function("calculate_interest_score", |b| {
        b.iter(|| calculate_interest_score(lat, lon, &heritage_sites, max_distance))
    });
}

fn bench_calculate_aerodromes_interest_scores(c: &mut Criterion) {
    let mut data = StringBasedWorldData::default();

    let mut group = c.benchmark_group("calculate_aerodromes_interest_scores");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);

    group.bench_function("calculate_aerodromes_interest_scores", |b| {
        b.iter(|| data.calculate_aerodromes_interest_scores())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_calculate_interest_score,
    bench_calculate_aerodromes_interest_scores
);
criterion_main!(benches);
