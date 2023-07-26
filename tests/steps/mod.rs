use crate::BddWorld;
use cucumber::{given, then};
use flyconomy::model::Timestamp;
use std::time::Duration;

pub mod base_management;
pub mod landing_rights_management;
pub mod plane_management;

#[given(regex = r"^the simulation is at timestamp (\d+)$")]
async fn the_simulation_is_at_timestamp(w: &mut BddWorld, timestamp: Timestamp) {
    assert!(
        w.simulation.environment.timestamp < timestamp,
        "The simulation timestamp is already over {}",
        timestamp
    );
    w.simulation.update(Duration::from_millis(
        (timestamp - w.simulation.environment.timestamp)
            .try_into()
            .unwrap(),
    ));
}

#[then(regex = r"^the simulation timestamp should be less than (\d+)$")]
async fn the_simulation_timestamp_should_be_less_than(w: &mut BddWorld, timestamp: Timestamp) {
    assert!(
        w.simulation.environment.timestamp < timestamp,
        "The simulation timestamp is not less than {}",
        timestamp
    );
}

#[given("the simulation is running")]
async fn the_simulation_is_running(w: &mut BddWorld) {
    w.simulation.time_multiplier = 1.0;
    w.simulation.update(Duration::from_secs(1));
}

#[then(regex = r"the simulation should have more than (\d+) cash")]
async fn the_simulation_should_have_more_than_cash(w: &mut BddWorld, cash: f64) {
    assert!(
        w.simulation
            .environment
            .company_finances
            .cash(w.simulation.environment.timestamp)
            > cash,
        "Simulation does not have more than {} cash",
        cash
    );
}

#[then(regex = r"the simulation should have more than (\d+) bases")]
async fn the_simulation_should_have_more_than_bases(w: &mut BddWorld, bases: usize) {
    assert!(
        w.simulation.environment.bases.len() > bases,
        "Simulation does not have more than {} bases",
        bases
    );
}

#[then(regex = r"the simulation should have exact (\d+) bases")]
async fn the_simulation_should_have_exact_bases(w: &mut BddWorld, bases: usize) {
    assert_eq!(
        w.simulation.environment.bases.len(),
        bases,
        "Simulation does not have exactly {} bases",
        bases
    );
}

#[then(regex = r"the simulation should have more than (\d+) airplanes")]
async fn the_simulation_should_have_more_than_airplanes(w: &mut BddWorld, airplanes: usize) {
    assert!(
        w.simulation.environment.planes.len() > airplanes,
        "Simulation does not have more than {} airplanes",
        airplanes
    );
}

#[then(regex = r"the simulation should have exact (\d+) airplanes")]
async fn the_simulation_should_have_exact_airplanes(w: &mut BddWorld, airplanes: usize) {
    assert_eq!(
        w.simulation.environment.planes.len(),
        airplanes,
        "Simulation does not have exactly {} airplanes",
        airplanes
    );
}

#[given(regex = r"^I have a starting cash of (\d+)$")]
async fn i_have_a_starting_cash_of(w: &mut BddWorld, cash: f64) {
    w.starting_cash = cash;
    w.simulation.environment.company_finances.income.clear();
    w.simulation.environment.company_finances.expenses.clear();
    w.simulation
        .environment
        .company_finances
        .add_income(0, cash);
}

#[then("I should get an InsufficientFunds error")]
async fn i_should_get_an_insufficient_funds_error(w: &mut BddWorld) {
    assert!(w.last_result.is_err());
}
