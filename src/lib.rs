#[cfg(feature = "ai")]
pub mod ai;
pub mod algorithms;
pub mod config;
pub mod game;
pub mod model;
pub mod simulation;
pub mod ui;
pub mod utils;

pub use simulation::replay::Replay;

use bevy::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn start() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flyconomy".into(),
                    resolution: (1280., 720.).into(),
                    prevent_default_event_handling: true,
                    canvas: Some("#bevy".to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin { ..default() }),
    );

    let level = "level1".to_string();
    let game_resource = game::GameResource::new(level);

    game::setup_game(&mut app, game_resource);
    log::info!("Starting game.");
    app.run()
}

pub fn start_from_replay(replay: Replay) {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flyconomy".into(),
                    resolution: (1280., 720.).into(),
                    prevent_default_event_handling: true,
                    canvas: Some("#bevy".to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin { ..default() }),
    );

    let game_resource = game::GameResource::from_replay(replay);

    game::setup_game(&mut app, game_resource);
    log::info!("Starting game from replay.");
    app.run()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn start_from_replay_string(replay_string: String) {
    let replay: Replay =
        serde_yaml::from_str(&replay_string).expect("Failed to deserialize replay.");
    start_from_replay(replay);
}
