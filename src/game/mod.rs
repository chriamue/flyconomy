use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod aerodrome;
mod camera;
pub mod earth3d;
pub mod flights;
mod game_state;
mod plane;
pub mod projection;
pub mod world_heritage_site;

#[cfg(feature = "ai")]
pub mod manager;

use bevy::prelude::IntoSystemConfigs;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
pub use game_state::GameState;

use crate::config::LevelConfig;
use crate::model::StringBasedWorldData;
use crate::{
    config::{AerodromeConfig, PlanesConfig},
    simulation::Simulation,
    ui, Replay,
};

#[derive(Resource)]
pub struct GameResource {
    pub level: String,
    pub simulation: Simulation,
    pub replay: Option<Replay>,
}

impl GameResource {
    pub fn new(level: String) -> Self {
        let level_config: LevelConfig = match level.as_str() {
            "default" => LevelConfig::default(),
            "level1" => serde_yaml::from_str(include_str!("../../assets/levels/level1.yaml"))
                .expect("Failed to load level1"),
            _ => panic!("Unknown level {}", level),
        };

        Self {
            level,
            simulation: Simulation::new(
                level_config.environment,
                Box::new(StringBasedWorldData::default()),
            ),
            replay: None,
        }
    }

    pub fn from_replay(replay: Replay) -> Self {
        let mut simulation = Simulation::new(
            replay.initial_config.clone(),
            Box::new(StringBasedWorldData::default()),
        );
        for timestamped_command in &replay.command_history {
            simulation.add_command_timed(timestamped_command.clone());
        }

        Self {
            level: String::from("replay"),
            simulation,
            replay: Some(replay),
        }
    }
}

impl Default for GameResource {
    fn default() -> Self {
        Self {
            level: String::from("default"),
            simulation: Simulation::new(
                LevelConfig::default().environment,
                Box::new(StringBasedWorldData::default()),
            ),
            replay: None,
        }
    }
}

#[derive(Default, Resource)]
pub struct ConfigResource {
    pub level_config: Option<LevelConfig>,
}

pub fn setup_game(app: &mut App, game_resource: GameResource) {
    app.add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(YamlAssetPlugin::<PlanesConfig>::new(&["yaml"]))
        .add_plugin(YamlAssetPlugin::<AerodromeConfig>::new(&[
            "aerodromes.json",
        ]))
        .add_state::<GameState>()
        .insert_resource(game_resource)
        .insert_resource(ConfigResource::default())
        .add_startup_system(setup_lights)
        .add_startup_system(load_config_assets)
        .add_systems((update_simulation_system,).in_set(OnUpdate(GameState::Playing)))
        .add_plugin(camera::CameraPlugin)
        .add_plugin(flights::FlightsPlugin)
        .add_plugin(aerodrome::AerodromePlugin)
        .add_plugin(world_heritage_site::WorldHeritageSitePlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(plane::PlanePlugin)
        .add_plugin(earth3d::Earth3dPlugin)
        .add_system(spin)
        .register_type::<Spin>();
    #[cfg(feature = "ai")]
    app.add_plugin(manager::ManagerPlugin);
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
        brightness: 0.3,
    });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(5.0, 0.5, 5.0),
            point_light: PointLight {
                intensity: 2000.0,
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
        < 10000.0
    {
        game_state_next_state.set(GameState::GameOver);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn start() {
    let level = "level1".to_string();
    let game_resource = GameResource::new(level);

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
    app.run()
}

pub fn start_from_replay(replay: Replay) {
    let game_resource = GameResource::from_replay(replay);

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
    app.run()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn start_from_replay_string(replay_string: String) {
    let replay: Replay =
        serde_yaml::from_str(&replay_string).expect("Failed to deserialize replay.");
    start_from_replay(replay);
}

#[derive(Resource)]
struct PlanesConfigHandle(Handle<PlanesConfig>);

#[derive(Resource)]
struct AerodromeConfigHandle(Handle<AerodromeConfig>);
