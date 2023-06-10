use bevy::prelude::{App, Query, Res, ResMut, Resource, Transform};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    game::{earth3d, projection::wgs84_to_xyz, ConfigResource, GameResource, GameState},
    model::{commands::BuyPlaneCommand, Aerodrome},
    overpass_importer::Element,
    simulation::Simulation,
};

pub fn add_ui_systems_to_app(app: &mut App) {
    app.insert_resource(UiInput {
        search_string: String::new(),
    });
    app.add_system(welcome_screen);
    app.add_system(game_over_screen);
    app.add_system(company_hud);
    app.add_system(planes_purchase_ui);
    app.add_system(aerodromes_ui);
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

pub fn game_over_screen(mut contexts: EguiContexts, mut game_resources: ResMut<GameResource>) {
    if !matches!(game_resources.game_state, GameState::GameOver) {
        return;
    }

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label("Game Over");

        // Display the final stats, like score, number of planes, etc.
        ui.label(format!(
            "Final Airline Value: ${:.2}",
            game_resources.simulation.environment.company_finances.cash
        ));
        ui.label(format!(
            "Total Planes: {}",
            game_resources.simulation.environment.planes.len()
        ));

        ui.label("Thank you for playing Flyconomy!");

        // Give the player the option to restart the game.
        if ui.button("Restart Game").clicked() {
            game_resources.game_state = GameState::Welcome;
            game_resources.simulation = Simulation::new(1_000_000.0);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Quit").clicked() {
            std::process::exit(0);
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
    config_resource: Res<ConfigResource>,
    mut game_resource: ResMut<GameResource>,
) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }
    if let Some(planes_config) = config_resource.planes_config.as_ref() {
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

pub fn aerodromes_ui(
    mut contexts: EguiContexts,
    config_resource: Res<ConfigResource>,
    game_resource: ResMut<GameResource>,
    mut search_input: ResMut<UiInput>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }
    if let Some(aerodromes_config) = config_resource.aerodrome_config.as_ref() {
        egui::Window::new("Aerodromes")
            .default_open(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Available Aerodromes:");
                ui.text_edit_singleline(&mut search_input.search_string);
                if let Ok(elements) = Element::from_json(&aerodromes_config.0) {
                    let aerodromes: Vec<Aerodrome> =
                        elements.into_iter().map(Aerodrome::from).collect();

                    for aerodrome in aerodromes
                        .iter()
                        .filter(|a| a.name.contains(&search_input.search_string))
                    {
                        if ui.selectable_label(false, &aerodrome.name).clicked() {
                            let aerodrome_position =
                                wgs84_to_xyz(aerodrome.lat, aerodrome.lon, 0.0)
                                    * earth3d::SCALE_FACTOR as f32;
                            for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                                pan_orbit.focus = aerodrome_position;
                            }
                        }
                    }
                } else {
                    ui.label("Error parsing aerodromes.");
                }
            });
    }
}

#[derive(Resource, Default)]
pub struct UiInput {
    pub search_string: String,
}
