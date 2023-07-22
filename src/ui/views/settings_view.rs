use crate::{
    game::{GameResource, GameState},
    ui::{
        components::{
            save_replay::{save_replay, UiInputReplayFilename},
            style_switch::{style_switch, StyleState},
        },
        layouts::left_layout,
    },
};
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut};
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
            (settings_view_system,)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiView::Settings)),
        );
    }
}

pub fn settings_view_system(
    mut contexts: EguiContexts,
    game_resource: Res<GameResource>,
    replay_filename: ResMut<UiInputReplayFilename>,
    style_state: ResMut<StyleState>,
) {
    left_layout("Settings").show(contexts.ctx_mut(), |ui| {
        style_switch(ui, style_state);
        ui.separator();
        save_replay(ui, game_resource, replay_filename)
    });
}
