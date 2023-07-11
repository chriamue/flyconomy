use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut, Resource};
use bevy_egui::{
    egui::{self, vec2, Align2, ComboBox},
    EguiContexts,
};

use crate::{
    game::{aerodrome::SelectedAerodrome, GameResource, GameState},
    model::{
        commands::{BuyPlaneCommand, SellPlaneCommand},
        AirPlane, PlaneType,
    },
};

use super::{aerodromes_ui::UiInput, UiState};

pub struct PlanesUiPlugin;

#[derive(Default, Resource)]
pub struct SelectedPlane {
    plane_type: Option<PlaneType>,
    pub airplane: Option<AirPlane>,
}

impl Plugin for PlanesUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiInput {
            search_string: String::new(),
        })
        .insert_resource(SelectedPlane::default())
        .add_systems(
            (planes_purchase_ui, airplanes_list_ui)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Aerodromes)),
        );
    }
}

pub fn planes_purchase_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    mut game_resource: ResMut<GameResource>,
    mut selected_plane: ResMut<SelectedPlane>,
) {
    let plane_types = game_resource
        .simulation
        .world_data_gateway
        .plane_types()
        .clone();
    if let Some(selected_aerodrome) = &selected_aerodrome.aerodrome {
        egui::Window::new("Buy Planes")
            .anchor(Align2::RIGHT_BOTTOM, vec2(0.0, 0.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Available Planes:");

                ComboBox::from_id_source("planes_list")
                    .selected_text(
                        selected_plane
                            .plane_type
                            .as_ref()
                            .map(|p| &p.name)
                            .unwrap_or(&"Select Plane".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for plane in plane_types {
                            ui.selectable_value(
                                &mut selected_plane.plane_type,
                                Some(plane.clone()),
                                &plane.name,
                            );
                        }
                    });

                if let Some(selected_plane) = &selected_plane.plane_type {
                    ui.separator();
                    ui.label(format!("Cost: ${:.2}", selected_plane.cost));
                    ui.label(format!(
                        "Monthly Income: ${:.2}",
                        selected_plane.monthly_income
                    ));
                    ui.label(format!("Range: {} km", selected_plane.range));
                    ui.label(format!("Speed: {} km/h", selected_plane.speed));
                    ui.label(format!("Capacity: {} passengers", selected_plane.seats));
                    ui.label(format!(
                        "Fuel Consumption: {} L/km",
                        selected_plane.fuel_consumption_per_km
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
                            plane_id: BuyPlaneCommand::generate_id(),
                            plane_type: selected_plane.clone(),
                            home_base_id: home_base_id.unwrap_or_default(),
                        };
                        game_resource.simulation.add_command(Box::new(buy_plane));
                    }
                }
            });
    }
}

pub fn airplanes_list_ui(
    mut contexts: EguiContexts,
    mut game_resource: ResMut<GameResource>,
    mut selected_airplane: ResMut<SelectedPlane>,
) {
    egui::Window::new("Airplanes")
        .pivot(Align2::RIGHT_CENTER)
        .default_open(true)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            let environment = &game_resource.simulation.environment;

            egui::ScrollArea::vertical().show(ui, |ui| {
                for airplane in &environment.planes {
                    let airplane_id = airplane.id;
                    let base = environment
                        .bases
                        .iter()
                        .find(|base| base.id == airplane.base_id);
                    let base_name = base.as_ref().map_or("", |base| &base.aerodrome.name);

                    let is_selected = selected_airplane
                        .airplane
                        .as_ref()
                        .map_or(false, |a| a.id == airplane_id);
                    if ui
                        .selectable_label(
                            is_selected,
                            format!("ID: {}, Base: {}", airplane_id, base_name),
                        )
                        .clicked()
                    {
                        selected_airplane.airplane = Some(airplane.clone());
                    }
                }
            });

            if let Some(airplane) = &selected_airplane.airplane {
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.heading("Selected Airplane Details");
                });
                ui.label(format!("ID: {}", airplane.id));
                ui.label(format!("Type: {}", airplane.plane_type.name));
                let base = environment
                    .bases
                    .iter()
                    .find(|base| base.id == airplane.base_id)
                    .unwrap();
                ui.label(format!("Base: {}", base.aerodrome.name));

                let (passengers, distance) =
                    environment.flights.iter().fold((0, 0.0), |acc, flight| {
                        if flight.airplane.id == airplane.id {
                            (
                                acc.0 + flight.calculate_booked_seats(),
                                acc.1 + flight.calculate_distance(),
                            )
                        } else {
                            acc
                        }
                    });
                ui.label(format!("Transported Passengers: {}", passengers));
                ui.label(format!("Total Distance: {:.3} km", distance));

                if ui.button("Sell Airplane").clicked() {
                    let cmd = SellPlaneCommand {
                        plane_id: airplane.id,
                    };
                    game_resource.simulation.add_command(Box::new(cmd));

                    selected_airplane.airplane = None;
                }
            }
        });
}
