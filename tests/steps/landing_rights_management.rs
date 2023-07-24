use crate::BddWorld;
use approx::assert_relative_eq;
use cucumber::{given, then, when};
use flyconomy::model::commands::{BuyLandingRightsCommand, Command};
use flyconomy::model::Aerodrome;

#[given(regex = r"^the cost to buy landing rights is (\d+)$")]
async fn the_cost_to_buy_landing_rights_is(w: &mut BddWorld, cost: f64) {
    w.simulation.environment.config.landing_rights_cost = cost;
}

#[when("I try to buy landing rights at an aerodrome")]
async fn i_try_to_buy_landing_rights(w: &mut BddWorld) {
    let aerodrome = Aerodrome::default();
    let cmd = BuyLandingRightsCommand {
        landing_rights_id: BuyLandingRightsCommand::generate_id(),
        aerodrome,
    };
    w.last_result = cmd.execute(&mut w.simulation.environment);
}

#[then(regex = r"^I should (successfully|fail to) buy the landing rights$")]
async fn i_should_result_buy_the_landing_rights(w: &mut BddWorld, result: String) {
    match &w.last_result {
        Ok(_) if result == "successfully" => (),
        Err(_e) if result == "fail to" => (),
        _ => panic!("Unexpected result when creating the base"),
    }
}

#[then("the number of landing rights should remain unchanged")]
async fn the_number_of_landing_rights_should_remain_unchanged(w: &mut BddWorld) {
    assert_eq!(
        w.simulation.environment.landing_rights.len(),
        w.starting_landing_rights_count
    );
}

#[then(regex = r"^my cash should be reduced by (\d+) if the rights were bought$")]
async fn my_cash_should_be_reduced_by_if_the_rights_were_bought(w: &mut BddWorld, reduction: f64) {
    if w.last_result.is_ok() {
        let new_cash = w
            .simulation
            .environment
            .company_finances
            .cash(w.simulation.environment.timestamp);

        assert_relative_eq!(new_cash, w.starting_cash - reduction, epsilon = 0.00001);
    }
}
