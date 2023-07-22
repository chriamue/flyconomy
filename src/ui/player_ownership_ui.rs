use bevy::prelude::{
    App, EventWriter, IntoSystemConfigs, OnUpdate, Plugin, Query, ResMut, Transform,
};
use bevy_egui::EguiContexts;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::game::{aerodrome::SelectedAerodromeChangeEvent, GameResource, GameState};

use super::{
    components::{
        bases::bases_list,
        landing_rights::{landing_rights_list, LandingRightsInput},
        planes::{planes_list, SelectedPlane},
    },
    layouts::right_layout,
    views::UiView,
};

pub struct PlayerOwnershipUiPlugin;

impl Plugin for PlayerOwnershipUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (player_ownership_info_ui,)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiView::Aerodromes)),
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
        bases_list(
            ui,
            &game_resource,
            &mut ev_selected_aerodrome_change,
            &mut pan_orbit_query,
        );

        ui.separator();

        landing_rights_list(
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
