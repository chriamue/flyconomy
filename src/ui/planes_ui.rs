use bevy::prelude::{App, Plugin, Res, ResMut, Resource};
use bevy_egui::egui::{self, ComboBox};

use crate::{
    game::{aerodrome::SelectedAerodrome, GameResource},
    model::{commands::BuyPlaneCommand, AirPlane, PlaneType},
};

use super::aerodromes_ui::UiInput;

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
        .insert_resource(SelectedPlane::default());
    }
}

pub fn planes_purchase_ui(
    ui: &mut egui::Ui,
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
        ui.vertical_centered(|ui| {
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
