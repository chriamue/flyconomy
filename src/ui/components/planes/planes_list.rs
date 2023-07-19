use bevy::prelude::ResMut;
use bevy_egui::egui;

use crate::{game::GameResource, model::commands::SellPlaneCommand, ui::planes_ui::SelectedPlane};

pub fn planes_list(
    ui: &mut egui::Ui,
    game_resource: &mut ResMut<GameResource>,
    selected_airplane: &mut ResMut<SelectedPlane>,
) {
    ui.label("Owned Airplanes:");

    if game_resource.simulation.environment.planes.is_empty() {
        ui.label("No airplanes owned");
        return;
    }

    let environment = &game_resource.simulation.environment;

    ui.vertical(|ui| {
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

        let (passengers, distance) = environment.flights.iter().fold((0, 0.0), |acc, flight| {
            if flight.airplane.id == airplane.id {
                (
                    acc.0 + flight.calculate_booked_seats(),
                    acc.1 + flight.calculate_total_distance(),
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
}
