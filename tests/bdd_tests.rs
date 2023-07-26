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
    starting_plane_count: usize,
    starting_cash: f64,
    last_result: Result<Option<String>, Box<dyn Error>>,
    last_plane_type: String,
    last_base_id: u64,
    landing_rights_id: u64,
}

impl Default for BddWorld {
    fn default() -> Self {
        let mut world = Self {
            simulation: Simulation::default(),
            starting_base_count: 0,
            starting_landing_rights_count: 0,
            starting_plane_count: 0,
            starting_cash: 0.0,
            last_result: Ok(None),
            last_plane_type: String::new(),
            last_base_id: 0,
            landing_rights_id: 0,
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
