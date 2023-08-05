use crate::game::world_heritage_site::SelectedWorldHeritageSite;
use crate::game::GameState;
use bevy::prelude::*;
use bevy::prelude::{App, Plugin, Update};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

pub struct WorldHeritageSiteUiPlugin;

impl Plugin for WorldHeritageSiteUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiInput {
            search_string: String::new(),
        })
        .add_systems(
            Update,
            (selected_site_info_ui_system,).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Resource, Default)]
pub struct UiInput {
    pub search_string: String,
}

fn selected_site_info_ui_system(
    mut contexts: EguiContexts,
    selected_site: Res<SelectedWorldHeritageSite>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if let Some(selected_site) = &selected_site.site {
        let alpha = (90.0 + selected_site.lon).to_radians();
        let beta = selected_site.lat.to_radians();

        egui::Window::new("UNESCO World Heritage Site")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                if ui
                    .selectable_label(false, format!("{}", selected_site.name))
                    .clicked()
                {
                    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                        pan_orbit.target_alpha = alpha as f32;
                        pan_orbit.target_beta = beta as f32;
                        pan_orbit.radius = Some(1.5);
                        pan_orbit.force_update = true;
                    }
                }
                ui.label(format!("Latitude: {:.4}", selected_site.lat));
                ui.label(format!("Longitude: {:.4}", selected_site.lon));

                // Additional details about the site can be added here
                ui.label(format!("{}", selected_site.description));
            });
    }
}
