use crate::game::aerodrome::SelectedAerodromeChangeEvent;
use crate::game::{ConfigResource, GameState};
use bevy::prelude::*;
use bevy::prelude::{App, OnUpdate, Plugin, ResMut};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

use super::UiInput;

pub struct AerodromesUiPlugin;

impl Plugin for AerodromesUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((aerodromes_ui_system,).in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn aerodromes_ui_system(
    mut contexts: EguiContexts,
    config_resource: Res<ConfigResource>,
    mut search_input: ResMut<UiInput>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if let Some(aerodromes) = config_resource.aerodromes.as_ref() {
        egui::Window::new("Aerodromes")
            .default_open(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Available Aerodromes:");
                ui.text_edit_singleline(&mut search_input.search_string);

                for aerodrome in aerodromes
                    .iter()
                    .filter(|a| a.name.contains(&search_input.search_string))
                {
                    if ui.selectable_label(false, &aerodrome.name).clicked() {
                        ev_selected_aerodrome_change
                            .send(SelectedAerodromeChangeEvent(aerodrome.clone()));
                        let alpha = (90.0 + aerodrome.lon).to_radians();
                        let beta = aerodrome.lat.to_radians();
                        for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                            pan_orbit.target_alpha = alpha as f32;
                            pan_orbit.target_beta = beta as f32;
                            pan_orbit.radius = Some(1.5);
                            pan_orbit.force_update = true;
                        }
                    }
                }
            });
    }
}
