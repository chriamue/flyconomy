use bevy::prelude::*;

mod game_state;
use bevy_egui::EguiPlugin;
pub use game_state::GameState;

use crate::ui;

#[derive(Default, Resource)]
pub struct GameResource {
    pub game_state: GameState,
}

pub fn setup_game(app: &mut App, game_resource: GameResource) {
    app.add_plugin(EguiPlugin)
        .insert_resource(game_resource)
        .add_startup_system(setup);
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
