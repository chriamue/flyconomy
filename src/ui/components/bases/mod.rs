use bevy::prelude::{EventWriter, Query, ResMut, Transform};
use bevy_egui::egui;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    game::{aerodrome::SelectedAerodromeChangeEvent, GameResource},
    model::Base,
};

pub fn base(
    ui: &mut egui::Ui,
    base: &Base,
    ev_selected_aerodrome_change: &mut EventWriter<SelectedAerodromeChangeEvent>,
    pan_orbit_query: &mut Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    ui.horizontal(|ui| {
        if ui
            .selectable_label(
                false,
                format!("{} ({})", base.aerodrome.name, base.aerodrome.code),
            )
            .clicked()
        {
            ev_selected_aerodrome_change.send(SelectedAerodromeChangeEvent(base.aerodrome.clone()));
            let alpha = (90.0 + base.aerodrome.lon).to_radians();
            let beta = base.aerodrome.lat.to_radians();
            for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                pan_orbit.target_alpha = alpha as f32;
                pan_orbit.target_beta = beta as f32;
                pan_orbit.radius = Some(1.5);
                pan_orbit.force_update = true;
            }
        }
        ui.label(format!("Airplanes: {}", base.airplane_ids.len()));
    });
}

pub fn bases_list(
    ui: &mut egui::Ui,
    game_resource: &ResMut<GameResource>,
    ev_selected_aerodrome_change: &mut EventWriter<SelectedAerodromeChangeEvent>,
    pan_orbit_query: &mut Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    ui.label("Owned Bases:");

    if game_resource.simulation.environment.bases.is_empty() {
        ui.label("No bases owned");
        return;
    }

    egui::ScrollArea::vertical()
        .id_source("bases_list")
        .show(ui, |ui| {
            for base in &game_resource.simulation.environment.bases {
                self::base(ui, base, ev_selected_aerodrome_change, pan_orbit_query);
            }
        });
}
