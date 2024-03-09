use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

pub mod aerodrome;
pub mod attraction;
mod camera;
pub mod earth3d;
pub mod flights;
mod game_state;
pub mod plane;
pub mod projection;
pub mod world_heritage_site;

#[cfg(feature = "ai")]
pub mod manager;

pub use game_state::GameState;

use crate::config::LevelConfig;
use crate::{simulation::Simulation, ui, Replay};

#[derive(Resource)]
pub struct GameResource {
    pub level: String,
    pub simulation: Simulation,
    pub replay: Option<Replay>,
    pub game_over_cash: f64,
}

impl GameResource {
    pub fn new(level: String) -> Self {
        let level_config: LevelConfig = match level.as_str() {
            "default" => LevelConfig::default(),
            "level1" => serde_yaml::from_str(include_str!("../../assets/levels/level1.yaml"))
                .expect("Failed to load level1"),
            _ => panic!("Unknown level {}", level),
        };
        let world_data_gateway = {
            #[cfg(not(feature = "web3"))]
            {
                crate::model::world_data::StringBasedWorldData::default()
            }
            #[cfg(feature = "web3")]
            {
                crate::model::world_data::web3_world_data::Web3WorldData::default()
            }
        };

        Self {
            level,
            simulation: Simulation::new(level_config.environment, Box::new(world_data_gateway)),
            replay: None,
            game_over_cash: 10_000.0,
        }
    }

    pub fn from_replay(replay: Replay) -> Self {
        let world_data_gateway = {
            #[cfg(not(feature = "web3"))]
            {
                crate::model::world_data::StringBasedWorldData::default()
            }
            #[cfg(feature = "web3")]
            {
                crate::model::world_data::web3_world_data::Web3WorldData::default()
            }
        };
        let mut simulation =
            Simulation::new(replay.initial_config.clone(), Box::new(world_data_gateway));
        for timestamped_command in &replay.command_history {
            simulation.add_command_timed(timestamped_command.clone());
        }

        Self {
            level: String::from("replay"),
            simulation,
            replay: Some(replay),
            ..Default::default()
        }
    }
}

impl Default for GameResource {
    fn default() -> Self {
        Self::new(String::from("default"))
    }
}

#[derive(Default, Resource)]
pub struct ConfigResource {
    pub level_config: Option<LevelConfig>,
}

pub fn setup_game(app: &mut App, game_resource: GameResource) {
    app.add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .init_state::<GameState>()
        .insert_resource(game_resource)
        .insert_resource(ConfigResource::default())
        .add_systems(Startup, (setup_lights, load_config_assets))
        .add_systems(
            Update,
            (update_simulation_system,).run_if(in_state(GameState::Playing)),
        )
        .add_plugins(camera::CameraPlugin)
        .add_plugins(flights::FlightsPlugin)
        .add_plugins(aerodrome::AerodromePlugin)
        .add_plugins(attraction::AttractionPlugin)
        .add_plugins(world_heritage_site::WorldHeritageSitePlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(plane::PlanePlugin)
        .add_plugins(earth3d::Earth3dPlugin)
        .add_systems(Update, spin)
        .register_type::<Spin>();
    #[cfg(feature = "ai")]
    app.add_plugins(manager::ManagerPlugin);

    log::info!("Game setup complete.");
}

#[derive(Component, PartialEq, Reflect)]
struct Spin {
    angular_velocity: f32,
    radius: f32,
    current_angle: f32,
}

fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &mut Spin)>) {
    for (mut transform, mut spin) in query.iter_mut() {
        spin.current_angle += spin.angular_velocity * time.delta_seconds();
        let (sin, cos) = spin.current_angle.sin_cos();
        transform.translation.x = cos * spin.radius;
        transform.translation.z = sin * spin.radius;
    }
}

fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 160.0,
    });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(5.0, 0.5, 5.0),
            point_light: PointLight {
                intensity: 2_000_000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Spin {
            angular_velocity: 0.02, // Angular velocity in radians per second
            radius: 5.0,            // Radius of the orbit
            current_angle: 0.0,
        });
}

fn load_config_assets(mut config_resource: ResMut<ConfigResource>) {
    let level_config: LevelConfig =
        serde_yaml::from_str(include_str!("../../assets/levels/level1.yaml")).unwrap();
    config_resource.level_config = Some(level_config);
}

fn update_simulation_system(
    mut game_resource: ResMut<GameResource>,
    time: Res<Time>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    game_resource.simulation.update(time.delta());
    if game_resource
        .simulation
        .environment
        .company_finances
        .cash(game_resource.simulation.environment.timestamp)
        < game_resource.game_over_cash
    {
        game_state_next_state.set(GameState::GameOver);
    }
}
