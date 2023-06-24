use crate::{
    game::{GameResource, GameState},
    simulation::replay::Replay,
};
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut, Resource};
use bevy_egui::{egui, EguiContexts};

pub struct ReplayPlugin;

impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiInputReplayFilename {
            replay_filename: "last.replay.yaml".to_string(),
        })
        .add_systems((save_replay_system,).in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn save_replay_system(
    mut contexts: EguiContexts,
    game_resource: Res<GameResource>,
    mut replay_filename: ResMut<UiInputReplayFilename>,
) {
    egui::Window::new("Save Replay")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Filename:");
                ui.text_edit_singleline(&mut replay_filename.replay_filename);
            });

            if ui.button("Save").clicked() {
                if !replay_filename.replay_filename.is_empty() {
                    // Create Replay struct and save to file.
                    let replay = Replay::new(
                        game_resource.simulation.environment.config.clone(),
                        game_resource.simulation.command_history.clone(),
                    );

                    if let Err(e) = replay.save_to_file(&replay_filename.replay_filename) {
                        println!("Failed to save replay: {:?}", e);
                    }
                }
            }
        });
}

#[derive(Resource, Default)]
pub struct UiInputReplayFilename {
    pub replay_filename: String,
}
