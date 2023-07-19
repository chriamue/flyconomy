use crate::game::GameState;
use bevy::prelude::*;
use bevy::prelude::{App, NextState, OnUpdate, Plugin, ResMut};
use bevy_egui::{egui, EguiContexts};

pub struct WelcomeScreenPlugin;

impl Plugin for WelcomeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((welcome_screen_system,).in_set(OnUpdate(GameState::Welcome)));
    }
}

pub fn welcome_screen_system(
    mut contexts: EguiContexts,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.label("");
            ui.heading("Welcome to Flyconomy!");
            ui.label("");

            ui.label(
                "Flyconomy is an economic simulation game where you manage an airline company.",
            );

            ui.label("Features:");
            ui.label("- Choose your home aerodrome and buy your first aircraft.");
            ui.label("- Plan flights by selecting an airplane and a destination aerodrome.");
            ui.label("- Calculate the estimated distance and profit for your planned flights.");
            ui.label("- Schedule your flights.");

            ui.label("Rules:");
            ui.label("- Start by choosing a home aerodrome and buying your first aircraft.");
            ui.label("- Plan flight routes between aerodromes.");
            ui.label("- Monitor fuel prices and adapt your routes.");
            ui.label("- Reinvest your profits to buy new planes and expand.");

            ui.label("Getting Started:");
            ui.label(
                "- Use the flight planning UI to select an airplane and a destination aerodrome.",
            );
            ui.label("- View the estimated distance and profit before scheduling your flight.");
            ui.label("- Click on 'Plan Flight' to schedule your flight.");

            if ui.button("Start Game").clicked() {
                game_state_next_state.set(GameState::Playing);
            }
        });
    });
}
