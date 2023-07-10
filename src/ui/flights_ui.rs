use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut, Resource};
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};
use chrono::{TimeZone, Utc};

use crate::{
    algorithms::calculate_interest_score,
    game::{aerodrome::SelectedAerodrome, ConfigResource, GameResource, GameState},
    model::{commands::ScheduleFlightCommand, Flight},
};

use super::UiState;

pub struct FlightsUiPlugin;

impl Plugin for FlightsUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FlightPlanningInput::default());
        app.add_systems(
            (flight_planning_ui, flight_list_ui)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Schedule)),
        );
    }
}

pub fn flight_planning_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    mut game_resource: ResMut<GameResource>,
    mut flight_planning_input: ResMut<FlightPlanningInput>,
    config_resource: Res<ConfigResource>,
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
                                    // interest score
                                    let heritage_sites: Vec<(f64, f64, f64)> = config_resource
                                        .world_heritage_sites
                                        .as_ref()
                                        .unwrap()
                                        .iter()
                                        .map(|site| (site.lat, site.lon, 1.0f64))
                                        .collect();
                                    let interest_score = calculate_interest_score(
                                        destination_aerodrome.lat,
                                        destination_aerodrome.lon,
                                        &heritage_sites,
                                        250_000.0,
                                    );

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
                                        interest_score,
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
                                            flight_id: ScheduleFlightCommand::generate_id(),
                                            airplane,
                                            origin_aerodrome,
                                            destination_aerodrome: destination_aerodrome.clone(),
                                            departure_time: game_resource
                                                .simulation
                                                .environment
                                                .timestamp,
                                            interest_score,
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
    pub selected_flight: Option<Flight>,
}

pub fn flight_list_ui(
    mut contexts: EguiContexts,
    mut game_resource: ResMut<GameResource>,
    mut flight_planning_input: ResMut<FlightPlanningInput>,
) {
    egui::Window::new("Flight List")
        .pivot(Align2::RIGHT_CENTER)
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let environment = &game_resource.simulation.environment;
            let mut new_flights = vec![];

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add scrolling feature
                for flight in environment.flights.iter().rev() {
                    let flight_id = flight.flight_id;
                    let from = &flight.origin_aerodrome.name;
                    let to = &flight.destination_aerodrome.name;
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
                ui.label(format!("Profit: ${:.2}", flight.calculate_profit()));
                ui.label(format!("Passengers: {}", flight.calculate_booked_seats()));
                ui.label(format!("Distance: {:.3} km", flight.calculate_distance()));
                ui.label(format!("Interest Score: {:.3}", flight.interest_score));
                if ui.button("Replicate Flight").clicked() {
                    let new_flight = ScheduleFlightCommand {
                        flight_id: ScheduleFlightCommand::generate_id(),
                        airplane: flight.airplane.clone(),
                        origin_aerodrome: flight.origin_aerodrome.clone(),
                        destination_aerodrome: flight.destination_aerodrome.clone(),
                        departure_time: game_resource.simulation.environment.timestamp,
                        interest_score: flight.interest_score,
                    };
                    new_flights.push(Box::new(new_flight));
                }
            }

            for flight in new_flights {
                game_resource.simulation.add_command(flight);
            }
        });
}
