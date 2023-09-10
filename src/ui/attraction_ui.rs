use crate::game::attraction::SelectedAttraction;
use crate::game::GameState;
use bevy::prelude::*;
use bevy::prelude::{App, Plugin, Update};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

pub struct AttractionUiPlugin;

impl Plugin for AttractionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (selected_attraction_info_ui_system,).run_if(in_state(GameState::Playing)),
        );
    }
}

fn selected_attraction_info_ui_system(
    mut contexts: EguiContexts,
    selected_attraction: Res<SelectedAttraction>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if let Some(selected_attraction) = &selected_attraction.attraction {
        let alpha = (90.0 + selected_attraction.lon).to_radians();
        let beta = selected_attraction.lat.to_radians();

        egui::Window::new("Attraction")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                if ui
                    .selectable_label(false, format!("{}", selected_attraction.name))
                    .clicked()
                {
                    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                        pan_orbit.target_alpha = alpha as f32;
                        pan_orbit.target_beta = beta as f32;
                        pan_orbit.radius = Some(1.5);
                        pan_orbit.force_update = true;
                    }
                }
                ui.label(format!("Latitude: {:.4}", selected_attraction.lat));
                ui.label(format!("Longitude: {:.4}", selected_attraction.lon));

                // Additional details about the attraction can be added here
                ui.label(format!("{}", selected_attraction.description));
            });
    }
}
