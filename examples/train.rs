use std::time::Duration;

use bevy::{
    prelude::{
        default, App, AssetPlugin, Commands, DespawnRecursiveExt, Entity, NextState, PluginGroup,
        Query, Res, ResMut, Resource, State, Transform,
    },
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use flyconomy::{
    ai::{AiManager, AiTrainerType},
    game::{plane::FlightVisual, setup_game, GameResource, GameState},
    model::{
        commands::{
            BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand, ScheduleFlightCommand,
        },
        Environment, StringBasedWorldData,
    },
    simulation::{Simulation, DEFAULT_TIME_MULTIPLIER},
    utils::find_aerodrome_by_code,
};

const TRAIN_ITERATIONS: usize = 1_000_000;

pub fn start_commands(simulation: &mut Simulation) {
    let innsbruck_code = "INN/LOWI";
    let innsbruck_aerodrome =
        find_aerodrome_by_code(simulation.world_data_gateway.aerodromes(), innsbruck_code)
            .expect("Paris aerodrome not found!");

    let frankfurt_code = "FRA/EDDF";
    let frankfurt_aerodrome =
        find_aerodrome_by_code(simulation.world_data_gateway.aerodromes(), frankfurt_code)
            .expect("Frankfurt aerodrome not found!");

    let create_base_command = CreateBaseCommand {
        base_id: CreateBaseCommand::generate_id(),
        aerodrome: innsbruck_aerodrome.clone(),
    };

    let buy_landing_rights_command = BuyLandingRightsCommand {
        landing_rights_id: BuyLandingRightsCommand::generate_id(),
        aerodrome: frankfurt_aerodrome.clone(),
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
        origin_aerodrome: innsbruck_aerodrome.clone(),
        stopovers: vec![frankfurt_aerodrome.clone()],
        departure_time: (simulation.elapsed_time + Duration::from_secs(1)).as_millis(),
    };

    simulation.add_command(Box::new(flight_command));

    simulation.update(Duration::from_secs(1));
}

pub fn train(simulation: &mut Simulation) -> AiManager {
    let mut ai_manager = AiManager::new(AiTrainerType::DQNAgentTrainer);

    ai_manager.train_simulation(&simulation);
    ai_manager.train(TRAIN_ITERATIONS as u32);
    ai_manager
}

pub fn start(simulation: Simulation) {
    let default_environment = simulation.environment.clone();
    let level = "level1".to_string();

    let mut game_resource = GameResource::new(level);
    game_resource.simulation = simulation;
    game_resource.game_over_cash = 0.0;

    let ai_manager = train(&mut game_resource.simulation);

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flyconomy".into(),
                    resolution: (1280., 720.).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    canvas: Some("#bevy".to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin { ..default() }),
    );

    setup_game(&mut app, game_resource);
    app.insert_resource(AiManagerResource {
        ai_manager,
        default_environment,
        min_cash: 80_000,
    });
    app.add_startup_system(set_initial_state);
    app.add_system(update_ai_manager_system);

    app.run()
}

fn set_initial_state(
    mut state: ResMut<NextState<GameState>>,
    mut game_resource: ResMut<GameResource>,
) {
    state.set(GameState::Playing);
    game_resource.simulation.time_multiplier = DEFAULT_TIME_MULTIPLIER * 25.0;
}

#[derive(Default, Resource)]
pub struct AiManagerResource {
    pub ai_manager: AiManager,
    pub default_environment: Environment,
    pub min_cash: u128,
}

pub fn update_ai_manager_system(
    mut ai_manager_resource: ResMut<AiManagerResource>,
    mut game_resource: ResMut<GameResource>,
    game_state: Res<State<GameState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    query: Query<(Entity, &FlightVisual, &mut Transform)>,
    mut commands: Commands,
) {
    if game_state.as_ref().0 == GameState::GameOver {
        game_state_next_state.set(GameState::Playing);
    }

    let simulation = &mut game_resource.simulation;
    if simulation
        .environment
        .company_finances
        .cash(simulation.environment.timestamp)
        < ai_manager_resource.min_cash as f64
    {
        simulation.environment = ai_manager_resource.default_environment.clone();
        simulation.elapsed_time = Duration::from_secs(0);
        simulation.command_history.clear();
        simulation.error_messages.clear();
        for (entity, _, _) in query.iter() {
            commands.entity(entity).remove::<FlightVisual>();
            commands.entity(entity).despawn_recursive();
        }
    } else {
        ai_manager_resource.ai_manager.update(simulation);
        let environment = &game_resource.simulation.environment;

        let world_data_gateway = &game_resource.simulation.world_data_gateway;
        let command = ai_manager_resource.ai_manager.best_command(
            environment,
            world_data_gateway.plane_types(),
            world_data_gateway.aerodromes(),
        );
        println!("Command: {:?}", command);
        if let Some(command) = command {
            game_resource.simulation.add_command(command);
        }
    }
}

pub fn main() {
    let mut simulation = Simulation::new(
        Default::default(),
        Box::new(StringBasedWorldData::default()),
    );
    simulation.setup();

    start_commands(&mut simulation);

    println!("Environment: {:#?}", simulation.environment);

    start(simulation);
}
