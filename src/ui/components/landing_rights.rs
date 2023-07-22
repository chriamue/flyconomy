use bevy::prelude::{EventWriter, Query, ResMut, Resource, Transform};
use bevy_egui::egui;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    game::{aerodrome::SelectedAerodromeChangeEvent, GameResource},
    model::{commands::SellLandingRightsCommand, LandingRights},
};

#[derive(Default, Resource)]
pub struct LandingRightsInput {
    pub selected_landing_rights: Option<LandingRights>,
}

pub fn landing_rights_list(
    ui: &mut egui::Ui,
    game_resource: &mut ResMut<GameResource>,
    landing_rights_input: &mut ResMut<LandingRightsInput>,
    ev_selected_aerodrome_change: &mut EventWriter<SelectedAerodromeChangeEvent>,
    pan_orbit_query: &mut Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    ui.label("Owned Landing Rights:");
    if game_resource
        .simulation
        .environment
        .landing_rights
        .is_empty()
    {
        ui.label("No landing rights owned");
        return;
    }
    egui::ScrollArea::vertical()
        .id_source("landing_rights_list")
        .max_height(300.0)
        .show(ui, |ui| {
            for landing_rights in &game_resource.simulation.environment.landing_rights {
                if ui
                    .selectable_label(
                        landing_rights_input
                            .selected_landing_rights
                            .as_ref()
                            .map_or(false, |selected| selected.id == landing_rights.id),
                        format!("{}", landing_rights.aerodrome.name),
                    )
                    .clicked()
                {
                    landing_rights_input.selected_landing_rights = Some(landing_rights.clone());
                }
            }
        });

    if let Some(landing_rights) = &landing_rights_input.selected_landing_rights {
        ui.label(format!(
            "Selected Aerodrome: {}",
            landing_rights.aerodrome.name
        ));
        if ui.button("Go to Aerodrome").clicked() {
            ev_selected_aerodrome_change.send(SelectedAerodromeChangeEvent(
                landing_rights.aerodrome.clone(),
            ));
            let alpha = (90.0 + landing_rights.aerodrome.lon).to_radians();
            let beta = landing_rights.aerodrome.lat.to_radians();
            for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                pan_orbit.target_alpha = alpha as f32;
                pan_orbit.target_beta = beta as f32;
                pan_orbit.radius = Some(1.5);
                pan_orbit.force_update = true;
            }
        }
        if ui.button("Sell Landing Rights").clicked() {
            let cmd = SellLandingRightsCommand {
                landing_rights_id: landing_rights.id.into(),
            };
            game_resource.simulation.add_command(Box::new(cmd));
            landing_rights_input.selected_landing_rights = None;
        }
    }
}
