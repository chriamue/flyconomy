use bevy::prelude::{App, Res, ResMut};
use bevy_egui::{egui, EguiContexts};

use crate::game::{GameResource, GameState};

pub fn add_ui_systems_to_app(app: &mut App) {
    app.add_system(welcome_screen);
    app.add_system(finances_hud);
}

pub fn welcome_screen(mut contexts: EguiContexts, mut game_resources: ResMut<GameResource>) {
    if !matches!(game_resources.game_state, GameState::Welcome) {
        return;
    }
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label("Welcome to Flyconomy!");

        ui.label("Flyconomy is an economic simulation game where you manage an airline company.");

        ui.label("Rules:");
        ui.label("- Start by choosing a home airport and buying your first aircraft.");
        ui.label("- Plan flight routes between airports.");
        ui.label("- Monitor fuel prices and adapt your routes.");
        ui.label("- Reinvest your profits to buy new planes and expand.");
        if ui.button("Start Game").clicked() {
            game_resources.game_state = GameState::Playing;
        }
    });
}

fn finances_hud(mut contexts: EguiContexts, game_resources: Res<GameResource>) {
    if !matches!(game_resources.game_state, GameState::Playing) {
        return;
    }

    egui::Window::new("Company Finances").show(contexts.ctx_mut(), |ui| {
        let finances = &game_resources.simulation.company_finances;
        ui.horizontal(|ui| {
            ui.label(format!("Cash: ${:.2}", finances.cash));
            ui.label(format!("Total Income: ${:.2}", finances.total_income));
            ui.label(format!("Total Expenses: ${:.2}", finances.total_expenses));
        });
    });
}
