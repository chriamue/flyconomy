use bevy::prelude::{App, EventWriter, OnUpdate, Query, Res, ResMut, Resource, States, Transform};
use bevy::prelude::{IntoSystemConfigs, Plugin};
use bevy_egui::egui::{vec2, Align2};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

mod aerodromes_ui;
mod analytics_ui;
mod game_over_screen;
mod hud;
mod messages;
mod replay;
mod simulation_control;
mod welcome_screen;
mod world_heritage_site_ui;

use crate::{
    game::{
        aerodrome::{SelectedAerodrome, SelectedAerodromeChangeEvent},
        ConfigResource, GameResource, GameState,
    },
    model::{
        commands::{BuyPlaneCommand, ScheduleFlightCommand},
        Flight,
    },
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiState>();
        app.insert_resource(FlightPlanningInput::default());
        app.add_plugin(hud::HudPlugin);
        app.add_plugin(welcome_screen::WelcomeScreenPlugin);
        app.add_plugin(game_over_screen::GameOverScreenPlugin);
        app.add_plugin(aerodromes_ui::AerodromesUiPlugin);
        app.add_plugin(world_heritage_site_ui::WorldHeritageSiteUiPlugin);
        app.add_plugin(messages::MessagesPlugin);
        app.add_plugin(replay::ReplayPlugin);
        app.add_plugin(simulation_control::SimulationControlPlugin);
        app.add_plugin(analytics_ui::AnalyticsPlugin);
        app.add_systems(
            (planes_purchase_ui, bases_info_ui)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Aerodromes)),
        );
        app.add_systems(
            (flight_planning_ui, bases_info_ui)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Schedule)),
        );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UiState {
    #[default]
    Aerodromes,
    Settings,
    Analytics,
    Schedule,
}

pub fn planes_purchase_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    config_resource: Res<ConfigResource>,
    mut game_resource: ResMut<GameResource>,
) {
    if let (Some(selected_aerodrome), Some(planes_config)) = (
        &selected_aerodrome.aerodrome,
        config_resource.planes_config.as_ref(),
    ) {
        egui::Window::new("Buy Planes")
            .anchor(Align2::RIGHT_BOTTOM, vec2(0.0, 0.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Available Planes:");

                for plane in &planes_config.planes {
                    ui.horizontal(|ui| {
                        ui.label(&plane.name);
                        ui.label(format!("Cost: ${:.2}", plane.cost));
                        ui.label(format!("Monthly Income: ${:.2}", plane.monthly_income));
                        ui.label(format!("Range: {} km", plane.range));
                        ui.label(format!("Speed: {} km/h", plane.speed));
                        ui.label(format!("Capacity: {} passengers", plane.seats));
                        ui.label(format!(
                            "Fuel Consumption: {} L/km",
                            plane.fuel_consumption_per_km
                        ));

                        if ui.button("Buy").clicked() {
                            let home_base_id = game_resource
                                .simulation
                                .environment
                                .bases
                                .iter()
                                .find(|base| base.aerodrome.id == selected_aerodrome.id)
                                .map(|base| base.id);

                            let buy_plane = BuyPlaneCommand {
                                plane_type: plane.clone(),
                                home_base_id: home_base_id.unwrap_or_default(),
                            };
                            game_resource.simulation.add_command(Box::new(buy_plane));
                        }
                    });
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
    egui::Window::new("Bases Info")
        .anchor(Align2::RIGHT_TOP, vec2(0.0, 100.0))
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

pub fn flight_planning_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    mut game_resource: ResMut<GameResource>,
    mut flight_planning_input: ResMut<FlightPlanningInput>,
) {
    if let Some(selected_aerodrome) = &selected_aerodrome.aerodrome {
        egui::Window::new("Flight Planning")
            .pivot(Align2::RIGHT_CENTER)
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
                                        departure_time: game_resource
                                            .simulation
                                            .environment
                                            .timestamp,
                                        arrival_time: None,
                                        state: Default::default(),
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
                                            departure_time: game_resource
                                                .simulation
                                                .environment
                                                .timestamp,
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
