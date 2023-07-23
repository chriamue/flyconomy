use crate::{game::GameResource, utils::timestamp_to_calendar_string};
use bevy::prelude::{App, Plugin, Res};
use bevy_egui::{egui, EguiContexts};

use super::layouts::{left_bottom_layout, right_bottom_layout};

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_error_messages)
            .add_system(show_event_messages);
    }
}

pub fn show_error_messages(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let elapsed_time = game_resource.simulation.elapsed_time.as_millis();
    let time_multiplier = game_resource.simulation.time_multiplier;

    if !game_resource.simulation.error_messages.is_empty() {
        right_bottom_layout("Error Book").show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical()
                .id_source("errors_list")
                .max_height(300.0)
                .show(ui, |ui| {
                    for message in &game_resource.simulation.error_messages {
                        if elapsed_time - message.0 < (8_000.0 * time_multiplier) as u128 {
                            let text = format!(
                                "{}, {}",
                                timestamp_to_calendar_string(message.0),
                                message.1
                            );
                            ui.label(text);
                        }
                    }
                });
        });
    }
}

pub fn show_event_messages(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let elapsed_time = game_resource.simulation.elapsed_time.as_millis();
    let time_multiplier = game_resource.simulation.time_multiplier;

    if !game_resource.simulation.event_messages.is_empty() {
        left_bottom_layout("Log Book").show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical()
                .id_source("messages_list")
                .max_height(300.0)
                .show(ui, |ui| {
                    for message in &game_resource.simulation.event_messages {
                        if elapsed_time - message.0 < (10_000.0 * time_multiplier) as u128 {
                            let text = format!(
                                "{}, {}",
                                timestamp_to_calendar_string(message.0),
                                message.1
                            );
                            ui.label(text);
                        }
                    }
                });
        });
    }
}
