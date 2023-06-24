use std::time::Duration;

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod aerodrome;
mod camera;
pub mod earth3d;
pub mod flights;
mod game_state;
mod plane;
pub mod projection;

use bevy::prelude::IntoSystemConfigs;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
pub use game_state::GameState;

use crate::config::LevelConfig;
use crate::{
    config::{parse_airport_csv, AerodromeConfig, PlanesConfig},
    model::Aerodrome,
    simulation::Simulation,
    ui, Replay,
};

#[derive(Resource)]
pub struct GameResource {
    pub level: String,
    pub simulation: Simulation,
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
            simulation: Simulation::new(level_config.environment),
        }
    }

    pub fn from_replay(replay: Replay) -> Self {
        let mut simulation = Simulation::new(replay.initial_config);
        let mut last_timestamp = 0u128;
        for timestamped_command in replay.command_history {
            simulation.execute_command(timestamped_command.command);
            let delta = timestamped_command.timestamp - last_timestamp;
            last_timestamp = timestamped_command.timestamp;
            simulation.update(Duration::from_millis(delta as u64));
        }

        Self {
            level: String::from("replay"),
            simulation,
        }
    }
}

impl Default for GameResource {
    fn default() -> Self {
        Self {
            level: String::from("default"),
            simulation: Simulation::new(LevelConfig::default().environment),
        }
    }
}

#[derive(Default, Resource)]
pub struct ConfigResource {
    pub plane_handle: Option<Handle<PlanesConfig>>,
    pub aerodrome_handle: Option<Handle<AerodromeConfig>>,
    pub planes_config: Option<PlanesConfig>,
    pub aerodrome_config: Option<AerodromeConfig>,
    pub aerodromes: Option<Vec<Aerodrome>>,
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
        .add_startup_system(setup)
        .add_startup_system(load_config_assets)
        .add_system(config_assets_loaded)
        .add_systems((update_simulation_system,).in_set(OnUpdate(GameState::Playing)))
        .add_plugin(camera::CameraPlugin)
        .add_plugin(flights::FlightsPlugin)
        .add_plugin(aerodrome::AerodromePlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(plane::PlanePlugin)
        .add_plugin(earth3d::Earth3dPlugin);
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 500.0, 100.0),
        point_light: PointLight {
            intensity: 1000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn load_config_assets(asset_server: Res<AssetServer>, mut config_resource: ResMut<ConfigResource>) {
    let handle = asset_server.load("planes.yaml");
    config_resource.plane_handle = Some(handle);

    let handle = asset_server.load("german.aerodromes.json");
    config_resource.aerodrome_handle = Some(handle);

    let aerodromes = parse_airport_csv(include_str!("../../assets/airports.dat"));
    config_resource.aerodromes = Some(aerodromes);

    let level_config: LevelConfig =
        serde_yaml::from_str(include_str!("../../assets/levels/level1.yaml")).unwrap();
    config_resource.level_config = Some(level_config);

    #[cfg(target_arch = "wasm32")]
    {
        let planes_config: PlanesConfig =
            serde_yaml::from_str(include_str!("../../assets/planes.yaml")).unwrap();
        config_resource.planes_config = Some(planes_config);

        let aerodrome_config: AerodromeConfig =
            serde_json::from_str(include_str!("../../assets/german.aerodromes.json")).unwrap();
        config_resource.aerodrome_config = Some(aerodrome_config);
    }
}

fn config_assets_loaded(
    mut config_resource: ResMut<ConfigResource>,
    planes_config_assets: Res<Assets<PlanesConfig>>,
    aerodrome_config_assets: Res<Assets<AerodromeConfig>>,
) {
    if config_resource.plane_handle.is_some() && config_resource.planes_config.is_none() {
        if let Some(handle) = config_resource.plane_handle.take() {
            if let Some(config) = planes_config_assets.get(&handle) {
                config_resource.planes_config = Some(config.clone());
            }
        }
    }

    if config_resource.aerodrome_handle.is_some() && config_resource.aerodrome_config.is_none() {
        if let Some(handle) = config_resource.aerodrome_handle.take() {
            if let Some(config) = aerodrome_config_assets.get(&handle) {
                config_resource.aerodrome_config = Some(config.clone());
            }
        }
    }
}

fn update_simulation_system(
    mut game_resource: ResMut<GameResource>,
    time: Res<Time>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    game_resource.simulation.update(time.delta());
    if game_resource.simulation.environment.company_finances.cash < 10000.0 {
        game_state_next_state.set(GameState::GameOver);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
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

#[derive(Resource)]
struct PlanesConfigHandle(Handle<PlanesConfig>);

#[derive(Resource)]
struct AerodromeConfigHandle(Handle<AerodromeConfig>);
