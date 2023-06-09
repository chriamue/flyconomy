use bevy::prelude::{App, Assets, Res, ResMut};
use bevy_egui::{egui, EguiContexts};

use crate::{
    config::PlanesConfig,
    game::{ConfigResource, GameResource, GameState},
    model::{commands::BuyPlaneCommand},
};

pub fn add_ui_systems_to_app(app: &mut App) {
    app.add_system(welcome_screen);
    app.add_system(company_hud);
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

fn company_hud(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }

    egui::Window::new("Company").show(contexts.ctx_mut(), |ui| {
        let environment = &game_resource.simulation.environment;
        ui.horizontal(|ui| {
            ui.label(format!("Cash: ${:.2}", environment.company_finances.cash));
            ui.label(format!("Planes: {}", environment.planes.len()));
            ui.label(format!(
                "Total Income: ${:.2}",
                environment.company_finances.total_income
            ));
            ui.label(format!(
                "Total Expenses: ${:.2}",
                environment.company_finances.total_expenses
            ));
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
                            let buy_plane = BuyPlaneCommand {
                                plane_type: plane.clone(),
                            };
                            game_resource.simulation.add_command(Box::new(buy_plane));
                        }
                    });
                }
            });
        }
    }
}
