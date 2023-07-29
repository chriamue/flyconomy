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
                "Flyconomy is an economic simulation game where you're in charge of your own airline company.",
            );
            ui.label("Your mission? Grow your airline empire through wise decision-making and strategic moves!");

            ui.label("Features:");
            ui.label("- Start by establishing your home base. Different aerodromes cost varying amounts, so manage your starting cash wisely!");
            ui.label("- Expand to other aerodromes by buying landing rights and be aware of their varying costs.");
            ui.label("- Boost your fleet by purchasing airplanes of different types. Each type has its own cost and capabilities.");
            ui.label("- Schedule flights between your bases. Ensure your airplanes can handle the distance, and don't forget to factor in departure times.");
            ui.label("- Monitor your financials closely. Insufficient funds can lead to failed expansions or purchases, but wise investments can lead to great rewards.");
            ui.label("- Stay vigilant! There are many challenges, such as aerodromes where bases already exist or selling assets that don’t exist.");

            ui.label("Tips for Success:");
            ui.label("- Always plan routes efficiently and watch out for fuel prices.");
            ui.label("- Reinvest your profits in expanding your fleet and bases.");
            ui.label("- Be aware of your assets. Know the planes you have, the landing rights you possess, and the bases you've established.");
            ui.label("- Every decision has a consequence. Think before you act, strategize, and lead your airline to success!");

            if ui.button("Dive In & Start Building Your Airline Empire!").clicked() {
                game_state_next_state.set(GameState::Playing);
            }
        });
    });
}
