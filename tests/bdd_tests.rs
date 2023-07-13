use cucumber::World;
use flyconomy::simulation::Simulation;
pub mod steps;

#[derive(Debug, World)]
pub struct BddWorld {
    simulation: Simulation,
}

impl Default for BddWorld {
    fn default() -> Self {
        let mut world = Self {
            simulation: Simulation::default(),
        };
        world.simulation.time_multiplier = 1.0;
        world
    }
}

#[tokio::main]
async fn main() {
    BddWorld::run("tests/features").await;
}
