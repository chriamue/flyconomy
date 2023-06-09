use bevy::prelude::*;

mod game_state;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_egui::EguiPlugin;
pub use game_state::GameState;

use crate::{config::PlanesConfig, simulation::Simulation, ui};

#[derive(Resource)]
pub struct GameResource {
    pub game_state: GameState,
    pub simulation: Simulation,
}

impl Default for GameResource {
    fn default() -> Self {
        Self {
            game_state: GameState::Welcome,
            simulation: Simulation::new(1_000_000.0),
        }
    }
}

#[derive(Default, Resource)]
pub struct ConfigResource {
    pub plane_handle: Option<Handle<PlanesConfig>>,
}

pub fn setup_game(app: &mut App, game_resource: GameResource) {
    app.add_plugin(EguiPlugin)
        .add_plugin(YamlAssetPlugin::<PlanesConfig>::new(&["yaml"]))
        .insert_resource(game_resource)
        .insert_resource(ConfigResource::default())
        .add_startup_system(setup)
        .add_startup_system(load_config_assets)
        .add_system(update_simulation_system);
    ui::add_ui_systems_to_app(app);
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
}

fn update_simulation_system(mut game_resource: ResMut<GameResource>, time: Res<Time>) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }
    game_resource.simulation.update(time.delta());
    if game_resource.simulation.environment.company_finances.cash < 10000.0 {
        game_resource.game_state = GameState::GameOver;
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn start() {
    let game_resource = GameResource::default();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Flyconomy".into(),
            resolution: (1000., 1000.).into(),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));
    setup_game(&mut app, game_resource);
    app.run()
}

#[derive(Resource)]
struct PlanesConfigHandle(Handle<PlanesConfig>);
