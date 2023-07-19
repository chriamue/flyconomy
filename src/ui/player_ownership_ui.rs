use bevy::prelude::{
    App, EventWriter, IntoSystemConfigs, OnUpdate, Plugin, Query, ResMut, Transform,
};
use bevy_egui::EguiContexts;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::game::{aerodrome::SelectedAerodromeChangeEvent, GameResource, GameState};

use super::{
    aerodromes_ui::{bases_info_ui, landing_rights_info_ui, LandingRightsInput},
    components::planes::planes_list,
    layouts::right_layout,
    planes_ui::SelectedPlane,
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
    right_layout("Player Ownership Info").show(contexts.ctx_mut(), |ui| {
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

        planes_list(ui, &mut game_resource, &mut selected_airplane);
    });
}
