use bevy::prelude::{
    in_state, App, EventWriter, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource, Transform,
    Update,
};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;
use chrono::{TimeZone, Utc};

use crate::{
    game::{
        aerodrome::{SelectedAerodrome, SelectedAerodromeChangeEvent},
        GameResource, GameState,
    },
    model::{commands::ScheduleFlightCommand, Aerodrome, Flight},
    ui::{
        components::{self, bases::bases_list},
        layouts::{left_layout, right_layout},
    },
};

use super::UiView;

pub struct ScheduleViewPlugin;

impl Plugin for ScheduleViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FlightPlanningInput::default());
        app.add_systems(
            Update,
            (flight_planning_ui, flight_list_ui)
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(UiView::Schedule)),
        );
    }
}

pub fn flight_planning_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    mut game_resource: ResMut<GameResource>,
    mut flight_planning_input: ResMut<FlightPlanningInput>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if let Some(selected_aerodrome) = &selected_aerodrome.aerodrome {
        right_layout("Flight Planning").show(contexts.ctx_mut(), |ui| {
            bases_list(
                ui,
                &game_resource,
                &mut ev_selected_aerodrome_change,
                &mut pan_orbit_query,
            );

            ui.separator();

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
                            flight_planning_input.selected_stopovers.clear();
                        }
                    }
                }
            } else {
                ui.label("No airplanes available at this base.");
            }

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

            let filtered_aerodromes = available_aerodromes
                .iter()
                .filter(|a| {
                    a.name
                        .to_lowercase()
                        .contains(&flight_planning_input.search_string.to_lowercase())
                })
                .cloned()
                .collect::<Vec<_>>();

            ui.label("Selected Stopover Aerodromes:");
            let mut last_aerodrome = selected_aerodrome.clone();
            for stopover_id in &flight_planning_input.selected_stopovers {
                if let Some(stopover_aerodrome) = available_aerodromes
                    .iter()
                    .find(|a| a.id == *stopover_id)
                    .as_deref()
                {
                    // calculate the distance from the last aerodrome to this stopover
                    let distance =
                        Flight::calculate_distance_between(&last_aerodrome, stopover_aerodrome);
                    ui.label(format!(
                        "{} (Distance: {:.3} km)",
                        stopover_aerodrome.name, distance
                    ));

                    // update the last aerodrome to the current stopover
                    last_aerodrome = (*stopover_aerodrome).clone();
                }
            }
            // calculate the distance from the last stopover back to the selected aerodrome
            let distance = Flight::calculate_distance_between(&last_aerodrome, selected_aerodrome);
            ui.label(format!(
                "{} (Distance: {:.3} km)",
                selected_aerodrome.name, distance
            ));

            ui.separator();

            ui.label("Select Stopover Aerodromes:");
            for aerodrome in filtered_aerodromes.iter() {
                let mut is_selected = flight_planning_input
                    .selected_stopovers
                    .contains(&aerodrome.id);
                if ui.checkbox(&mut is_selected, &aerodrome.name).changed() {
                    if is_selected {
                        flight_planning_input.selected_stopovers.push(aerodrome.id);
                    } else {
                        flight_planning_input
                            .selected_stopovers
                            .retain(|&x| x != aerodrome.id);
                    }
                }
            }

            if let Some(selected_airplane_id) = flight_planning_input.selected_airplane_id {
                if let (Some(base), Some(airplane)) = (
                    environment
                        .bases
                        .iter()
                        .find(|base| base.aerodrome.id == selected_aerodrome.id),
                    environment
                        .planes
                        .iter()
                        .find(|plane| plane.id == selected_airplane_id),
                ) {
                    let origin_aerodrome = base.aerodrome.clone();

                    let stopovers: Vec<Aerodrome> = flight_planning_input
                        .selected_stopovers
                        .iter()
                        .map(|&id| {
                            available_aerodromes
                                .iter()
                                .find(|a| a.id == id)
                                .cloned()
                                .unwrap()
                                .clone()
                        })
                        .collect();

                    if !stopovers.is_empty() {
                        let flight = Flight {
                            flight_id: 0,
                            airplane: airplane.clone(),
                            origin_aerodrome: origin_aerodrome.clone(),
                            stopovers: stopovers.clone(),
                            departure_time: game_resource.simulation.environment.timestamp,
                            segment_departure_time: game_resource.simulation.environment.timestamp,
                            arrival_time: None,
                            state: Default::default(),
                        };

                        components::Flight::new(&flight);

                        if ui.button("Plan Flight").clicked() {
                            let schedule_flight = ScheduleFlightCommand {
                                flight_id: ScheduleFlightCommand::generate_id(),
                                airplane: airplane.clone(),
                                origin_aerodrome,
                                stopovers,
                                departure_time: game_resource.simulation.environment.timestamp,
                            };
                            game_resource
                                .simulation
                                .add_command(Box::new(schedule_flight));
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
    pub selected_stopovers: Vec<u64>,
    pub selected_flight: Option<Flight>,
}
pub fn flight_list_ui(
    mut contexts: EguiContexts,
    mut game_resource: ResMut<GameResource>,
    mut flight_planning_input: ResMut<FlightPlanningInput>,
) {
    left_layout("Flights").show(contexts.ctx_mut(), |ui| {
        let environment = &game_resource.simulation.environment;
        let mut new_flights = vec![];

        egui::ScrollArea::vertical()
            .id_source("flight_list")
            .max_height(300.0)
            .show(ui, |ui| {
                for flight in environment.flights.iter().rev() {
                    let flight_id = flight.flight_id;
                    let from = &flight.origin_aerodrome.name;

                    let to = flight.stopovers.iter().fold(from.clone(), |acc, stopover| {
                        format!("{}, {}", acc, stopover.name)
                    });

                    let date_time = flight.departure_time;

                    let start_of_2000: i64 = 946684800000;
                    let timestamp: i64 = start_of_2000 + date_time as i64;
                    let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();

                    let is_selected = flight_planning_input
                        .selected_flight
                        .as_ref()
                        .map_or(false, |f| f.flight_id == flight_id);
                    if ui
                        .selectable_label(
                            is_selected,
                            format!(
                                "Flight ID: {}, From: {}, To: {}, Departure: {}",
                                flight_id,
                                from,
                                to,
                                datetime.format("%Y-%m-%d %H:%M").to_string()
                            ),
                        )
                        .clicked()
                    {
                        flight_planning_input.selected_flight = Some(flight.clone());
                    }
                }
            });

        if let Some(flight) = &flight_planning_input.selected_flight {
            components::Flight::new(flight);
            if ui.button("Replicate Flight").clicked() {
                let new_flight = ScheduleFlightCommand {
                    flight_id: ScheduleFlightCommand::generate_id(),
                    airplane: flight.airplane.clone(),
                    origin_aerodrome: flight.origin_aerodrome.clone(),
                    stopovers: flight.stopovers.clone(),
                    departure_time: game_resource.simulation.environment.timestamp,
                };
                new_flights.push(Box::new(new_flight));
            }
        }

        for flight in new_flights {
            game_resource.simulation.add_command(flight);
        }
    });
}
