use super::{analytics_ui, UiState};
use crate::game::manager::GameManagerType;
#[cfg(feature = "ai")]
use crate::game::manager::GameManagers;
use crate::game::GameResource;
use crate::game::GameState;
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut};
use bevy_egui::{
    egui::{vec2, Align2, Window},
    EguiContexts,
};
use strum::IntoEnumIterator;

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
    mut game_managers: ResMut<GameManagers>,
    game_resource: Res<GameResource>,
) {
    use bevy_egui::egui::ProgressBar;

    Window::new("Managers")
        .anchor(Align2::RIGHT_TOP, vec2(0.0, 0.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let error_indicator = game_resource.simulation.environment.get_errors_indicator();
            ui.label(format!("Managers - Error Indicator {}", error_indicator));
            let max_error_indicator = 100; // Define a reasonable maximum for the error indicator
            ui.add(
                ProgressBar::new(error_indicator as f32 / max_error_indicator as f32)
                    .text(format!("{}/{}", error_indicator, max_error_indicator)),
            );

            ui.separator();

            for manager_type in GameManagerType::iter() {
                if ui.button(format!("Hire {:?}", manager_type)).clicked() {
                    let new_manager = manager_type.create_manager();
                    game_managers.managers.push(new_manager);
                }
            }

            ui.separator();

            let mut to_remove = Vec::new();
            for (idx, manager) in game_managers.managers.iter_mut().enumerate() {
                ui.collapsing(format!("Manager #{}", manager.id), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Fire").clicked() {
                            to_remove.push(idx);
                        }
                        if ui
                            .button(if manager.is_working {
                                "Stop Working"
                            } else {
                                "Start Working"
                            })
                            .clicked()
                        {
                            manager.is_working = !manager.is_working;
                        }
                        if ui.button("Train").clicked() {
                            manager
                                .ai_manager
                                .train_simulation(&game_resource.simulation);
                        }
                    });

                    ui.label(format!("AI Manager ID: {:#?}", manager.id));
                    ui.label(&format!("Manager Action: {}", manager.manager_action));
                    ui.label(if manager.is_working {
                        "Working"
                    } else {
                        "Not Working"
                    });
                });
            }

            to_remove.sort_unstable_by(|a, b| b.cmp(a));
            for idx in to_remove {
                game_managers.managers.remove(idx);
            }
        });
}
