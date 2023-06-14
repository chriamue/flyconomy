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
        commands::{
            BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand, ScheduleFlightCommand,
        },
        Base, Flight,
    },
    simulation::Simulation,
};

pub fn add_ui_systems_to_app(app: &mut App) {
    app.insert_resource(UiInput {
        search_string: String::new(),
    });
    app.insert_resource(FlightPlanningInput::default());
    app.add_system(welcome_screen);
    app.add_system(game_over_screen);
    app.add_system(company_hud);
    app.add_system(planes_purchase_ui);
    app.add_system(aerodromes_ui);
    app.add_system(bases_info_ui);
    app.add_system(selected_aerodrome_info_ui);
    app.add_system(flight_planning_ui);
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
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
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
                        ev_selected_aerodrome_change
                            .send(SelectedAerodromeChangeEvent(base.aerodrome.clone()));
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

pub fn flight_planning_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    mut game_resource: ResMut<GameResource>,
    mut flight_planning_input: ResMut<FlightPlanningInput>,
) {
    if let Some(selected_aerodrome) = &selected_aerodrome.aerodrome {
        egui::Window::new("Flight Planning")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                ui.label(format!("Selected Aerodrome: {}", selected_aerodrome.name));

                ui.label("Select Airplane:");
                let environment = &game_resource.simulation.environment;
                let base = environment
                    .bases
                    .iter()
                    .find(|base| base.aerodrome.id == selected_aerodrome.id);

                if let Some(base) = base {
                    for airplane_id in &base.airplane_ids {
                        let airplane = environment
                            .planes
                            .iter()
                            .find(|plane| &plane.id == airplane_id);

                        if let Some(airplane) = airplane {
                            if ui
                                .selectable_label(
                                    flight_planning_input.selected_airplane_id == Some(airplane.id),
                                    format!(
                                        "Airplane ID: {}, Type: {}",
                                        airplane.id, airplane.plane_type.name
                                    ),
                                )
                                .clicked()
                            {
                                flight_planning_input.selected_airplane_id = Some(airplane.id);
                            }
                        }
                    }
                } else {
                    ui.label("No airplanes available at this base.");
                }

                ui.label("Select Destination Aerodrome:");
                ui.text_edit_singleline(&mut flight_planning_input.search_string);

                let mut available_aerodromes: Vec<_> = environment
                    .bases
                    .iter()
                    .map(|base| &base.aerodrome)
                    .collect();

                available_aerodromes.extend(
                    environment
                        .landing_rights
                        .iter()
                        .map(|landing_rights| &landing_rights.aerodrome),
                );

                available_aerodromes.sort_by_key(|a| a.id);
                available_aerodromes.dedup_by_key(|a| a.id);

                let filtered_aerodromes: Vec<_> = available_aerodromes
                    .iter()
                    .filter(|a| a.name.contains(&flight_planning_input.search_string))
                    .collect();

                for aerodrome in filtered_aerodromes.iter() {
                    if ui
                        .selectable_label(
                            flight_planning_input.selected_destination_id == Some(aerodrome.id),
                            &aerodrome.name,
                        )
                        .clicked()
                    {
                        flight_planning_input.selected_destination_id = Some(aerodrome.id);
                    }
                }

                if let Some(selected_airplane_id) = flight_planning_input.selected_airplane_id {
                    if let Some(selected_destination_id) =
                        flight_planning_input.selected_destination_id
                    {
                        if let Some(base) = environment
                            .bases
                            .iter()
                            .find(|base| base.aerodrome.id == selected_aerodrome.id)
                        {
                            if let Some(airplane) = environment
                                .planes
                                .iter()
                                .find(|plane| plane.id == selected_airplane_id)
                                .cloned()
                            {
                                let origin_aerodrome = base.aerodrome.clone();

                                let destination_aerodrome = available_aerodromes
                                    .iter()
                                    .find(|a| a.id == selected_destination_id)
                                    .cloned();

                                if let Some(destination_aerodrome) = destination_aerodrome {
                                    let flight = Flight {
                                        flight_id: 0, // Dummy ID as it is only being used for calculation
                                        airplane: airplane.clone(),
                                        origin_aerodrome: origin_aerodrome.clone(),
                                        destination_aerodrome: destination_aerodrome.clone(),
                                    };

                                    let profit = flight.calculate_profit();

                                    let distance_in_kilometers = flight.calculate_distance();
                                    ui.label(format!(
                                        "Estimated Distance: {:.3} km",
                                        distance_in_kilometers
                                    ));
                                    ui.label(format!("Estimated Profit: ${:.2}", profit));

                                    if ui.button("Plan Flight").clicked() {
                                        let schedule_flight = ScheduleFlightCommand {
                                            airplane,
                                            origin_aerodrome,
                                            destination_aerodrome: destination_aerodrome.clone(),
                                        };
                                        game_resource
                                            .simulation
                                            .add_command(Box::new(schedule_flight));
                                    }
                                }
                            }
                        }
                    }
                }
            });
    }
}

#[derive(Resource, Default)]
pub struct FlightPlanningInput {
    pub search_string: String,
    pub selected_airplane_id: Option<u64>,
    pub selected_destination_id: Option<u64>,
}
