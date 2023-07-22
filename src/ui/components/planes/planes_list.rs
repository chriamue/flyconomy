use super::{plane, SelectedPlane};
use crate::{game::GameResource, model::commands::SellPlaneCommand};
use bevy::prelude::ResMut;
use bevy_egui::egui;

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

        plane(
            ui,
            airplane,
            &game_resource.simulation.environment.flights,
            &game_resource.simulation.environment.bases,
        );

        if ui.button("Sell Airplane").clicked() {
            let cmd = SellPlaneCommand {
                plane_id: airplane.id,
            };
            game_resource.simulation.add_command(Box::new(cmd));

            selected_airplane.airplane = None;
        }
    }
}
