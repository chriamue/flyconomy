use bevy::prelude::App;
use bevy_egui::{egui, EguiContexts};

pub fn add_ui_systems_to_app(app: &mut App) {
    app.add_system(welcome_screen);
}

pub fn welcome_screen(mut contexts: EguiContexts) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label("Welcome to Flyconomy!");

        ui.label("Flyconomy is an economic simulation game where you manage an airline company.");

        ui.label("Rules:");
        ui.label("- Start by choosing a home airport and buying your first aircraft.");
        ui.label("- Plan flight routes between airports.");
        ui.label("- Monitor fuel prices and adapt your routes.");
        ui.label("- Reinvest your profits to buy new planes and expand.");

        if ui.button("Start Game").clicked() {}
    });
}
