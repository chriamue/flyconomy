use bevy::prelude::{App, EventWriter, Query, Res, ResMut, Resource, Transform};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

mod scores;

use crate::{
    game::{
        aerodrome::{SelectedAerodrome, SelectedAerodromeChangeEvent},
        ConfigResource, GameResource, GameState,
    },
    model::{
        commands::{BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand},
        Base,
    },
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
    app.add_system(bases_info_ui);
    app.add_system(selected_aerodrome_info_ui);
    scores::add_scores_systems_to_app(app);
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

        ui.label(format!(
            "Final Airline Value: ${:.2}",
            game_resources.simulation.environment.company_finances.cash
        ));
        ui.label(format!(
            "Total Planes: {}",
            game_resources.simulation.environment.planes.len()
        ));

        ui.label("Thank you for playing Flyconomy!");

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

    egui::Window::new("Company")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
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
                            home_base_id: 0,
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
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }
    if let Some(aerodromes) = config_resource.aerodromes.as_ref() {
        egui::Window::new("Aerodromes")
            .default_open(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Available Aerodromes:");
                ui.text_edit_singleline(&mut search_input.search_string);

                for aerodrome in aerodromes
                    .iter()
                    .filter(|a| a.name.contains(&search_input.search_string))
                {
                    if ui.selectable_label(false, &aerodrome.name).clicked() {
                        ev_selected_aerodrome_change
                            .send(SelectedAerodromeChangeEvent(aerodrome.clone()));
                        let alpha = (90.0 + aerodrome.lon).to_radians();
                        let beta = aerodrome.lat.to_radians();
                        for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                            pan_orbit.target_alpha = alpha as f32;
                            pan_orbit.target_beta = beta as f32;
                            pan_orbit.radius = Some(1.5);
                            pan_orbit.force_update = true;
                        }
                    }
                }
            });
    }
}

pub fn bases_info_ui(
    mut contexts: EguiContexts,
    game_resource: Res<GameResource>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if !matches!(game_resource.game_state, GameState::Playing) {
        return;
    }

    egui::Window::new("Bases Info")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let environment = &game_resource.simulation.environment;
            ui.label("Owned Bases:");

            for base in &environment.bases {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(false, format!("Aerodrome: {}", base.aerodrome.name))
                        .clicked()
                    {
                        let alpha = (90.0 + base.aerodrome.lon).to_radians();
                        let beta = base.aerodrome.lat.to_radians();
                        for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                            pan_orbit.target_alpha = alpha as f32;
                            pan_orbit.target_beta = beta as f32;
                            pan_orbit.radius = Some(1.5);
                            pan_orbit.force_update = true;
                        }
                    }
                    ui.label(format!("Number of Airplanes: {}", base.airplane_ids.len()));
                });
            }
        });
}

#[derive(Resource, Default)]
pub struct UiInput {
    pub search_string: String,
}
fn selected_aerodrome_info_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    mut game_resource: ResMut<GameResource>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if let Some(selected_aerodrome) = &selected_aerodrome.aerodrome {
        let (is_base, base) = {
            let environment = &game_resource.simulation.environment;

            let is_base = environment
                .bases
                .iter()
                .any(|base| base.aerodrome.id == selected_aerodrome.id);

            let base: Option<Base> = environment
                .bases
                .iter()
                .find(|base| base.aerodrome.id == selected_aerodrome.id)
                .cloned();

            (is_base, base)
        };

        egui::Window::new("Selected Aerodrome")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                if ui
                    .selectable_label(false, format!("{}", selected_aerodrome.name))
                    .clicked()
                {
                    let alpha = (90.0 + selected_aerodrome.lon).to_radians();
                    let beta = selected_aerodrome.lat.to_radians();
                    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                        pan_orbit.target_alpha = alpha as f32;
                        pan_orbit.target_beta = beta as f32;
                        pan_orbit.radius = Some(1.5);
                        pan_orbit.force_update = true;
                    }
                }
                ui.label(format!("Latitude: {:.4}", selected_aerodrome.lat));
                ui.label(format!("Longitude: {:.4}", selected_aerodrome.lon));

                if is_base {
                    ui.label("This is one of your bases.");
                    if let Some(base) = base {
                        ui.label("Airplanes at this base:");
                        for airplane_id in &base.airplane_ids {
                            let airplane = game_resource
                                .simulation
                                .environment
                                .planes
                                .iter()
                                .find(|plane| &plane.id == airplane_id);
                            if let Some(airplane) = airplane {
                                ui.label(format!(
                                    "Airplane ID: {}, Type: {}",
                                    airplane.id, airplane.plane_type.name
                                ));
                            }
                        }
                    }
                } else {
                    ui.label("You do not have a base at this aerodrome.");

                    if ui.button("Create Base").clicked() {
                        let buy_plane = CreateBaseCommand {
                            aerodrome: selected_aerodrome.clone(),
                        };
                        game_resource.simulation.add_command(Box::new(buy_plane));
                    }
                    if ui.button("Buy Landing Rights").clicked() {
                        let buy_plane = BuyLandingRightsCommand {
                            aerodrome: selected_aerodrome.clone(),
                        };
                        game_resource.simulation.add_command(Box::new(buy_plane));
                    }
                }
            });
    }
}
