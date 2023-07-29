use crate::BddWorld;
use approx::assert_relative_eq;
use cucumber::{given, then, when};
use flyconomy::model::commands::Command;
use flyconomy::model::commands::CreateBaseCommand;
use flyconomy::model::StringBasedWorldData;
use flyconomy::model::Aerodrome;
use flyconomy::utils::find_best_fit_aerodrome_by_name_or_code;
use flyconomy::model::WorldDataGateway;
use flyconomy::model::commands::CreateBaseError;

#[given(regex = r"^the cost to create a base at an aerodrome with (\d+) passengers is (\d+)$")]
async fn the_cost_to_create_a_base_is(w: &mut BddWorld, passengers: u64, base_cost: f64) {
    let aerodrome = Aerodrome {
        passengers: Some(passengers),
        ..Aerodrome::default()
    };
    let cmd = CreateBaseCommand {
        base_id: CreateBaseCommand::generate_id(),
        aerodrome,
    };
    assert_eq!(cmd.base_cost(&w.simulation.environment), base_cost);
}

#[when("I try to create a base at the aerodrome")]
async fn i_try_to_create_a_base(w: &mut BddWorld) {
    let aerodrome = Aerodrome::default();
    let base_id = CreateBaseCommand::generate_id();
    let cmd = CreateBaseCommand { base_id, aerodrome };
    w.last_result = cmd.execute(&mut w.simulation.environment);
    w.last_base_id = base_id;
}

#[given("I created a base at the aerodrome")]
async fn i_created_a_base(w: &mut BddWorld) {
    let aerodrome = Aerodrome::default();
    let base_id = CreateBaseCommand::generate_id();
    let cmd = CreateBaseCommand { base_id, aerodrome };
    w.last_result = cmd.execute(&mut w.simulation.environment);
    w.last_base_id = base_id;
}

#[then(regex = r"^I should (successfully|fail to) create the base$")]
async fn i_should_result_create_the_base(w: &mut BddWorld, result: String) {
    match &w.last_result {
        Ok(_) if result == "successfully" => (),
        Err(_e) if result == "fail to" => (),
        _ => panic!("Unexpected result when creating the base"),
    }
}

#[then(regex = r"^my cash should be reduced by (\d+) if the base was created$")]
async fn my_cash_should_be_reduced_by_if_the_base_was_created(w: &mut BddWorld, reduction: f64) {
    if w.last_result.is_ok() {
        let new_cash = w
            .simulation
            .environment
            .company_finances
            .cash(w.simulation.environment.timestamp);

        let expected_cash = w.starting_cash - reduction;
        assert_relative_eq!(new_cash, expected_cash, epsilon = 1e-2, max_relative = 1e-2);
    }
}

#[then("the number of bases should remain unchanged")]
async fn the_number_of_bases_should_remain_unchanged(w: &mut BddWorld) {
    assert_eq!(w.simulation.environment.bases.len(), w.starting_base_count);
}

#[given(regex = r#"^I have a base at "([^"]+)"$"#)]
async fn i_have_a_base_at(w: &mut BddWorld, airport_name: String) {
    let world_data = StringBasedWorldData::default();
    let aerodrome = find_best_fit_aerodrome_by_name_or_code(world_data.aerodromes(), &airport_name).unwrap();
    let base_id = CreateBaseCommand::generate_id();
    let cmd = CreateBaseCommand { base_id, aerodrome };
    w.last_result = cmd.execute(&mut w.simulation.environment);
    w.last_base_id = base_id;
}

#[when(regex = r#"^I try to create another base at "([^"]+)"$"#)]
async fn i_try_to_create_another_base_at(w: &mut BddWorld, aerodrome_name: String) {
    let world_data = StringBasedWorldData::default();
    let aerodrome = find_best_fit_aerodrome_by_name_or_code(world_data.aerodromes(), &aerodrome_name).unwrap();
    let base_id = CreateBaseCommand::generate_id();
    let cmd = CreateBaseCommand { base_id, aerodrome };
    w.last_result = cmd.execute(&mut w.simulation.environment);
    w.last_base_id = base_id;
}

#[then(regex = r#"^I should get a BaseAlreadyExists error$"#)]
async fn i_should_get_a_base_already_exists_error(w: &mut BddWorld) {
    match &w.last_result {
        Err(err) => {
            if let Some(specific_err) = err.downcast_ref::<CreateBaseError>() {
                if !matches!(*specific_err, CreateBaseError::BaseAlreadyExists { .. }) {
                    panic!("Expected a BaseAlreadyExists error but got a different error");
                }
            } else {
                panic!("Expected a BaseAlreadyExists error but got a different error type");
            }
        }
        _ => panic!("Expected an error but got a successful result"),
    }
}


#[then(regex = "my cash should remain unchanged")]
async fn my_cash_should_remain_unchanged(w: &mut BddWorld) {
    let new_cash = w
        .simulation
        .environment
        .company_finances
        .cash(w.simulation.environment.timestamp);
    assert_relative_eq!(new_cash, w.starting_cash, epsilon = 1e-2, max_relative = 1e-2);
}
