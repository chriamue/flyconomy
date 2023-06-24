use crate::{
    config::PlanesConfig,
    model::{
        commands::{
            BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand, ScheduleFlightCommand,
        },
        Aerodrome,
    },
};

use super::*;

#[test]
fn test_simulation() {
    let mut simulation = Simulation::new(Default::default());
    simulation.setup();

    let paris_aerodrome =
        Aerodrome::new(0, 49.012798, 2.55, "Paris, Charles de Gaulle".to_string());

    let frankfurt_aerodrome = Aerodrome::new(
        1,
        50.033333,
        8.570556,
        "Frankfurt am Main Airport".to_string(),
    );

    let planes_config: PlanesConfig =
        serde_yaml::from_str(include_str!("../../assets/planes.yaml")).unwrap();

    let create_base_command = CreateBaseCommand {
        aerodrome: frankfurt_aerodrome.clone(),
    };

    let buy_landing_rights_command = BuyLandingRightsCommand {
        aerodrome: paris_aerodrome.clone(),
    };

    simulation.add_command(Box::new(create_base_command));
    simulation.update(Duration::from_secs(1));

    simulation.add_command(Box::new(buy_landing_rights_command));
    simulation.update(Duration::from_secs(1));

    let buy_plane_command = BuyPlaneCommand {
        plane_type: planes_config.planes[0].clone(),
        home_base_id: simulation.environment.bases[0].id,
    };

    simulation.add_command(Box::new(buy_plane_command));

    simulation.update(Duration::from_secs(1));

    assert_eq!(simulation.environment.planes.len(), 1);
    assert_eq!(simulation.environment.bases.len(), 1);
    assert_eq!(simulation.environment.landing_rights.len(), 1);
    assert_eq!(simulation.environment.flights.len(), 0);

    let flight_command = ScheduleFlightCommand {
        airplane: simulation.environment.planes[0].clone(),
        origin_aerodrome: frankfurt_aerodrome.clone(),
        destination_aerodrome: paris_aerodrome.clone(),
        departure_time: (simulation.elapsed_time + Duration::from_secs(1)).as_secs(),
    };

    simulation.add_command(Box::new(flight_command));

    simulation.update(Duration::from_secs(1));

    assert_eq!(simulation.environment.planes.len(), 1);
    assert_eq!(simulation.environment.flights.len(), 1);
}
