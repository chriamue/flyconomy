use crate::{
    game::{GameResource, GameState},
    ui::{
        components::{
            config::{identity_alias, style_switch::StyleSwitch},
            save_replay::{save_replay, UiInputReplayFilename},
        },
        layouts::left_layout,
    },
};
use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, ResMut, Resource, Update};
use bevy_egui::EguiContexts;

use super::UiView;

pub struct SettingsViewPlugin;

impl Plugin for SettingsViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StyleState { is_dark: true });
        app.insert_resource(UiInputReplayFilename {
            replay_filename: "last.replay.yaml".to_string(),
        });
        app.add_systems(
            Update,
            (settings_view_system,)
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(UiView::Settings)),
        );
    }
}

#[derive(Default, Resource)]
pub struct StyleState {
    pub is_dark: bool,
}

pub fn settings_view_system(
    mut contexts: EguiContexts,
    mut game_resource: ResMut<GameResource>,
    replay_filename: ResMut<UiInputReplayFilename>,
    mut style_state: ResMut<StyleState>,
) {
    left_layout("Settings").show(contexts.ctx_mut(), |ui| {
        if ui.add(StyleSwitch::new(style_state.is_dark)).clicked() {
            style_state.is_dark = !style_state.is_dark;
        }
        ui.separator();

        ui.add(identity_alias::IdentityAlias::new(
            &mut game_resource.simulation.environment.identity,
        ));

        ui.separator();
        save_replay(ui, game_resource.into(), replay_filename)
    });
}
