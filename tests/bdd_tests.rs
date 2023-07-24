use cucumber::World;
use flyconomy::simulation::Simulation;
pub mod steps;
use cucumber::WriterExt;
use std::boxed::Box;
use std::error::Error;
use std::result::Result;

#[derive(Debug, World)]
pub struct BddWorld {
    simulation: Simulation,
    starting_base_count: usize,
    starting_landing_rights_count: usize,
    starting_cash: f64,
    last_result: Result<Option<String>, Box<dyn Error>>,
}

impl Default for BddWorld {
    fn default() -> Self {
        let mut world = Self {
            simulation: Simulation::default(),
            starting_base_count: 0,
            starting_landing_rights_count: 0,
            starting_cash: 0.0,
            last_result: Ok(None),
        };
        world.simulation.time_multiplier = 1.0;
        world
    }
}

#[tokio::main]
async fn main() {
    BddWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(
            cucumber::writer::Basic::raw(std::io::stdout(), cucumber::writer::Coloring::Never, 0)
                .summarized()
                .assert_normalized(),
        )
        .run_and_exit("tests/features")
        .await;
}
