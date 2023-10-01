use crate::game::{GameResource, GameState};
use crate::simulation::Simulation;
use bevy::prelude::{in_state, App, IntoSystemConfigs, NextState, Plugin, ResMut, Update};
use bevy_egui::{egui, EguiContexts};

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (game_over_screen_system,).run_if(in_state(GameState::GameOver)),
        );
    }
}

pub fn game_over_screen_system(
    mut contexts: EguiContexts,
    mut game_resources: ResMut<GameResource>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.label("");
            ui.heading("Game Over!");
            ui.label("");

            ui.label(format!(
                "Final Airline Value: ${:.2}",
                game_resources
                    .simulation
                    .environment
                    .company_finances
                    .cash(game_resources.simulation.environment.timestamp)
            ));
            ui.label(format!(
                "Total Planes: {}",
                game_resources.simulation.environment.planes.len()
            ));

            ui.label("Thank you for playing Flyconomy!");

            if ui.button("Restart Game").clicked() {
                game_resources.simulation = Simulation::new(
                    Default::default(),
                    #[cfg(not(feature = "web3"))]
                    Box::new(crate::model::world_data::StringBasedWorldData::default()),
                    #[cfg(feature = "web3")]
                    Box::new(crate::model::world_data::web3_world_data::Web3WorldData::default()),
                );
                game_state_next_state.set(GameState::Welcome);
            }
            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });
    });
}
