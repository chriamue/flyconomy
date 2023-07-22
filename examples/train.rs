use std::time::Duration;

use flyconomy::{
    ai::AiManager,
    model::{
        commands::{
            BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand, ScheduleFlightCommand,
        },
        Aerodrome, StringBasedWorldData,
    },
    simulation::Simulation,
};

pub fn start_commands(simulation: &mut Simulation) {
    let paris_aerodrome = Aerodrome::new(
        1381,
        49.012798,
        2.55,
        "Paris, Charles de Gaulle".to_string(),
        "CDG/LFPG".to_string(),
    );

    let frankfurt_aerodrome = Aerodrome::new(
        339,
        50.033333,
        8.570556,
        "Frankfurt am Main Airport".to_string(),
        "FRA/EDDF".to_string(),
    );

    let create_base_command = CreateBaseCommand {
        base_id: CreateBaseCommand::generate_id(),
        aerodrome: frankfurt_aerodrome.clone(),
    };

    let buy_landing_rights_command = BuyLandingRightsCommand {
        landing_rights_id: BuyLandingRightsCommand::generate_id(),
        aerodrome: paris_aerodrome.clone(),
    };

    simulation.add_command(Box::new(create_base_command));
    simulation.update(Duration::from_secs(1));

    simulation.add_command(Box::new(buy_landing_rights_command));
    simulation.update(Duration::from_secs(1));

    let buy_plane_command = BuyPlaneCommand {
        plane_id: BuyPlaneCommand::generate_id(),
        plane_type: simulation.world_data_gateway.plane_types()[0].clone(),
        home_base_id: simulation.environment.bases[0].id,
    };

    simulation.add_command(Box::new(buy_plane_command));

    simulation.update(Duration::from_secs(1));

    let flight_command = ScheduleFlightCommand {
        flight_id: ScheduleFlightCommand::generate_id(),
        airplane: simulation.environment.planes[0].clone(),
        origin_aerodrome: frankfurt_aerodrome.clone(),
        stopovers: vec![paris_aerodrome.clone()],
        departure_time: (simulation.elapsed_time + Duration::from_secs(1)).as_millis(),
    };

    simulation.add_command(Box::new(flight_command));

    simulation.update(Duration::from_secs(1));
}

pub fn train(simulation: &mut Simulation) {
    let mut ai_manager = AiManager::default();
    ai_manager.train_simulation(&simulation);
    ai_manager.train(1000);
}

pub fn main() {
    let mut simulation = Simulation::new(
        Default::default(),
        Box::new(StringBasedWorldData::default()),
    );
    simulation.setup();

    start_commands(&mut simulation);

    train(&mut simulation);

    println!("Environment: {:#?}", simulation.environment);
}
