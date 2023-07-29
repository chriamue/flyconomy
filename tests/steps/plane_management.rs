use crate::BddWorld;
use cucumber::{given, then, when};
use flyconomy::model::commands::{BuyPlaneCommand, Command, SellPlaneCommand};
use flyconomy::model::AirPlane;
use flyconomy::model::StringBasedWorldData;
use flyconomy::model::WorldDataGateway;

#[given(regex = r"^the cost to buy a plane of type (Small|Medium) Plane is (\d+)$")]
async fn the_cost_to_buy_a_plane_is(w: &mut BddWorld, plane_type: String, cost: f64) {
    let cost_as_f32 = cost as f32;

    let data = StringBasedWorldData::default();

    let plane = data
        .plane_types()
        .iter()
        .find(|p| p.name == plane_type || p.name == format!("{} Plane", plane_type));

    match plane {
        Some(plane) => {
            w.last_plane_type = plane.name.clone();
            assert_eq!(plane.cost, cost_as_f32)
        }
        None => panic!("Unsupported plane type"),
    };
}

#[when("I try to buy the plane")]
async fn i_try_to_buy_the_plane(w: &mut BddWorld) {
    let data = StringBasedWorldData::default();
    let plane_type = data
        .plane_types()
        .iter()
        .find(|p| p.name == w.last_plane_type)
        .unwrap();
    let cmd = BuyPlaneCommand {
        plane_id: BuyPlaneCommand::generate_id(),
        home_base_id: w.last_base_id,
        plane_type: plane_type.clone(),
    };
    w.last_result = cmd.execute(&mut w.simulation.environment);
}

#[then(regex = r"^I should (successfully|fail to) buy the plane$")]
async fn i_should_result_buy_the_plane(w: &mut BddWorld, result: String) {
    match &w.last_result {
        Ok(_) if result == "successfully" => (),
        Err(_e) if result == "fail to" => (),
        _ => panic!("Unexpected result when buying the plane"),
    }
}

#[then("the number of planes in my fleet should remain unchanged")]
async fn the_number_of_planes_should_remain_unchanged(w: &mut BddWorld) {
    assert_eq!(
        w.simulation.environment.planes.len(),
        w.starting_plane_count
    );
}

#[then(regex = r"^the simulation should have exact (\d+) airplane$")]
async fn the_simulation_should_have_exact_n_airplanes(w: &mut BddWorld, planes: u32) {
    assert_eq!(w.simulation.environment.planes.len(), planes as usize);
}

#[then(regex = r"^my cash should be reduced by (\d+) if the plane was bought$")]
async fn cash_reduced_by_amount_if_plane_bought(w: &mut BddWorld, plane_cost: f64) {
    match &w.last_result {
        Ok(_) => {
            let expected_cash = w.starting_cash - plane_cost;
            assert_eq!(
                w.simulation
                    .environment
                    .company_finances
                    .cash(w.simulation.environment.timestamp),
                expected_cash
            );
        }
        Err(_) => (), // Do nothing if the plane was not bought
    }
}

#[given(regex = r"^I own a plane with ID (\d+) of type (.+)$")]
async fn i_own_a_plane_with_id_and_type(w: &mut BddWorld, plane_id: u64, plane_type_name: String) {
    let data = StringBasedWorldData::default();
    let plane_type = data
        .plane_types()
        .iter()
        .find(|p| p.name == plane_type_name)
        .cloned()
        .expect("Plane type not found!");

    let base_id = w.last_base_id; // Assuming the plane should belong to the last created base in the world state.

    let plane = AirPlane {
        id: plane_id,
        base_id,
        plane_type,
    };

    w.simulation.environment.planes.push(plane);
    w.last_plane_id = plane_id;
}

#[when(regex = r"^I try to sell the plane with ID (\d+)$")]
async fn i_try_to_sell_the_plane_with_id(w: &mut BddWorld, plane_id: u64) {
    let cmd = SellPlaneCommand { plane_id };
    w.last_result = cmd.execute(&mut w.simulation.environment);
}

#[then(regex = r"^I should (successfully|fail to) sell the plane$")]
async fn i_should_result_sell_the_plane(w: &mut BddWorld, result: String) {
    match &w.last_result {
        Ok(_) if result == "successfully" => (),
        Err(_) if result == "fail to" => (),
        _ => panic!("Unexpected result when selling the plane"),
    }
}

#[then(regex = r"^my cash should increase by (\d+) if the plane was sold$")]
async fn cash_increased_by_amount_if_plane_sold(w: &mut BddWorld, plane_cost: f64) {
    match &w.last_result {
        Ok(_) => {
            let expected_cash = w.starting_cash + plane_cost;
            assert_eq!(
                w.simulation
                    .environment
                    .company_finances
                    .cash(w.simulation.environment.timestamp),
                expected_cash
            );
        }
        Err(_) => (), // Do nothing if the plane was not sold
    }
}
