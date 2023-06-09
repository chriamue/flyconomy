use bevy::prelude::{App, Assets, Res, ResMut};
use bevy_egui::{egui, EguiContexts};

use crate::{
    config::PlanesConfig,
    game::{ConfigResource, GameResource, GameState},
};

pub fn add_ui_systems_to_app(app: &mut App) {
    app.add_system(welcome_screen);
    app.add_system(finances_hud);
    app.add_system(planes_purchase_ui);
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

fn finances_hud(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }

    egui::Window::new("Company Finances").show(contexts.ctx_mut(), |ui| {
        let finances = &game_resource.simulation.company_finances;
        ui.horizontal(|ui| {
            ui.label(format!("Cash: ${:.2}", finances.cash));
            ui.label(format!("Total Income: ${:.2}", finances.total_income));
            ui.label(format!("Total Expenses: ${:.2}", finances.total_expenses));
        });
    });
}

pub fn planes_purchase_ui(
    mut contexts: EguiContexts,
    planes_assets: Res<Assets<PlanesConfig>>,
    config_resource: Res<ConfigResource>,
    mut game_resource: ResMut<GameResource>,
) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }
    if let Some(handle) = &config_resource.plane_handle {
        if let Some(planes_config) = planes_assets.get(handle) {
            egui::Window::new("Buy Planes").show(contexts.ctx_mut(), |ui| {
                ui.label("Available Planes:");

                for plane in &planes_config.planes {
                    ui.horizontal(|ui| {
                        ui.label(&plane.name);
                        ui.label(format!("Cost: ${:.2}", plane.cost));
                        ui.label(format!("Monthly Income: ${:.2}", plane.monthly_income));
                        ui.label(format!(
                            "Monthly Operating Cost: ${:.2}",
                            plane.monthly_operating_cost
                        ));

                        if ui.button("Buy").clicked() {
                            // Implement the logic for buying a plane here.
                            // You can access the game_resource to modify player's finances.
                            if game_resource.simulation.company_finances.cash >= plane.cost.into() {
                                game_resource.simulation.company_finances.cash -= plane.cost as f64;
                                game_resource.simulation.planes.push(plane.clone());
                                // Add other logic as needed for adding the plane to the player's assets
                            } else {
                                // Optionally, display a message that the player doesn't have enough cash
                            }
                        }
                    });
                }
            });
        }
    }
}
