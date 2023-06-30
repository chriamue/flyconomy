use crate::game::{GameResource, GameState};
use crate::simulation::Simulation;
use bevy::prelude::*;
use bevy::prelude::{App, NextState, OnUpdate, Plugin, ResMut};
use bevy_egui::{egui, EguiContexts};

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((game_over_screen_system,).in_set(OnUpdate(GameState::GameOver)));
    }
}

pub fn game_over_screen_system(
    mut contexts: EguiContexts,
    mut game_resources: ResMut<GameResource>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label("Game Over");

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
            game_resources.simulation = Simulation::new(Default::default());
            game_state_next_state.set(GameState::Welcome);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Quit").clicked() {
            std::process::exit(0);
        }
    });
}
