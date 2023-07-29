use crate::BddWorld;
use cucumber::{given, then, when};
use flyconomy::model::commands::{
    BuyPlaneCommand, Command, ScheduleFlightCommand, ScheduleFlightError,
};
use flyconomy::model::AirPlane;
use flyconomy::model::PlaneType;
use flyconomy::model::StringBasedWorldData;
use flyconomy::model::WorldDataGateway;
use flyconomy::utils::find_best_fit_aerodrome_by_name_or_code;

#[given(regex = r#"^I have an airplane with ID (\d+) located at airport named "([^"]+)"$"#)]
async fn i_have_an_airplane_at_airport_with_name(
    w: &mut BddWorld,
    airplane_id: u64,
    origin_name: String,
) {
    let world_data = StringBasedWorldData::default();
    let origin =
        find_best_fit_aerodrome_by_name_or_code(world_data.aerodromes(), &origin_name).unwrap();

    let base = w
        .simulation
        .environment
        .bases
        .iter()
        .find(|base| base.aerodrome.id == origin.id)
        .unwrap();

    let cmd = BuyPlaneCommand {
        plane_id: airplane_id,
        home_base_id: base.id,
        plane_type: PlaneType::default(),
    };
    w.last_result = cmd.execute(&mut w.simulation.environment);
    w.last_plane_id = cmd.plane_id;
}

#[when(
    regex = r#"^I try to schedule a flight using airplane with ID (\d+) from airport named "([^"]+)" to airport named "([^"]+)" with departure time (\d+)$"#
)]
async fn i_try_to_schedule_a_flight_using_airplane_from_airport_to_airport_with_time(
    w: &mut BddWorld,
    airplane_id: u64,
    origin_name: String,
    destination_name: String,
    departure_time: u64,
) {
    let world_data = StringBasedWorldData::default();
    let origin =
        find_best_fit_aerodrome_by_name_or_code(world_data.aerodromes(), &origin_name).unwrap();
    let destination =
        find_best_fit_aerodrome_by_name_or_code(world_data.aerodromes(), &destination_name)
            .unwrap();

    let base = w
        .simulation
        .environment
        .bases
        .iter()
        .find(|base| base.aerodrome.id == origin.id)
        .unwrap();

    let plane = AirPlane {
        id: airplane_id,
        base_id: base.id,
        ..AirPlane::default()
    };

    let cmd = ScheduleFlightCommand {
        flight_id: ScheduleFlightCommand::generate_id(),
        airplane: plane,
        origin_aerodrome: origin,
        stopovers: vec![destination],
        departure_time: departure_time.into(),
    };
    w.last_result = cmd.execute(&mut w.simulation.environment);
}

#[then(regex = r#"^I should (successfully|fail to) schedule the flight$"#)]
async fn i_should_result_schedule_the_flight(w: &mut BddWorld, result: String) {
    match &w.last_result {
        Ok(_) if result == "successfully" => (),
        Err(_) if result == "fail to" => (),
        _ => panic!(
            "Unexpected result when scheduling the flight: {:?}",
            w.last_result
        ),
    }
}

#[then(regex = r#"^I should get "([^"]+)" if the flight wasn't scheduled$"#)]
async fn i_should_get_error_message_if_the_flight_wasnt_scheduled(
    w: &mut BddWorld,
    error_message: String,
) {
    if let Err(err) = &w.last_result {
        if let Some(specific_err) = err.downcast_ref::<ScheduleFlightError>() {
            println!("specific_err: {:?}", specific_err);
            let actual_message = match *specific_err {
                ScheduleFlightError::AirplaneInUse => "Airplane is already in use",
                ScheduleFlightError::DistanceBeyondRange => {
                    "Distance is beyond the airplane's range"
                }
                _ => panic!("Unexpected error when scheduling the flight"),
            };
            assert_eq!(actual_message, error_message);
        } else {
            panic!("Expected a ScheduleFlightError but got a different error type");
        }
    }
}
