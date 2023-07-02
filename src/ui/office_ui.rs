#[cfg(feature = "ai")]
use crate::game::manager::ManagerAction;
use crate::game::GameState;
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut};
use bevy_egui::{
    egui::{vec2, Align2, Window},
    EguiContexts,
};

use super::{analytics_ui, UiState};
use crate::game::GameResource;

pub struct OfficePlugin;

impl Plugin for OfficePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                analytics_ui::company_hud_system,
                analytics_ui::show_cash_history,
                #[cfg(feature = "ai")]
                show_manager_action_system,
            )
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Office)),
        );
    }
}

#[cfg(feature = "ai")]
pub fn show_manager_action_system(
    mut contexts: EguiContexts,
    mut manager_action: ResMut<ManagerAction>,
    game_resource: Res<GameResource>,
) {
    Window::new("Manager Action")
        .anchor(Align2::RIGHT_TOP, vec2(0.0, 0.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            if ui.button("Toggle Work").clicked() {
                manager_action.is_working = !manager_action.is_working;
            }

            if manager_action.is_working {
                if ui.button("Replace Manager").clicked() {
                    manager_action
                        .ai_manager
                        .train_simulation(&game_resource.simulation);
                }
                ui.label("Manager is currently working");
            } else {
                ui.label("Manager is currently not working");
            }

            ui.label(&manager_action.manager_action);
        });
}
