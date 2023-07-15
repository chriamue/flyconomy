use bevy::prelude::{
    App, EventWriter, IntoSystemConfigs, OnUpdate, Plugin, Query, ResMut, Transform,
};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::game::{aerodrome::SelectedAerodromeChangeEvent, GameResource, GameState};

use super::{
    aerodromes_ui::{bases_info_ui, landing_rights_info_ui, LandingRightsInput},
    planes_ui::{airplanes_list_info_ui, SelectedPlane},
    UiState,
};

pub struct PlayerOwnershipUiPlugin;

impl Plugin for PlayerOwnershipUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (player_ownership_info_ui,)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Aerodromes)),
        );
    }
}

fn player_ownership_info_ui(
    mut contexts: EguiContexts,
    mut game_resource: ResMut<GameResource>,
    mut selected_airplane: ResMut<SelectedPlane>,
    mut landing_rights_input: ResMut<LandingRightsInput>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    egui::SidePanel::right("player_ownership_info_ui")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Player Ownership Info");
            });

            ui.separator();
            bases_info_ui(
                ui,
                &game_resource,
                &mut ev_selected_aerodrome_change,
                &mut pan_orbit_query,
            );

            ui.separator();

            landing_rights_info_ui(
                ui,
                &mut game_resource,
                &mut landing_rights_input,
                &mut ev_selected_aerodrome_change,
                &mut pan_orbit_query,
            );

            ui.separator();

            airplanes_list_info_ui(ui, &mut game_resource, &mut selected_airplane);
        });
}
