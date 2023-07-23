use crate::game::GameResource;
use bevy::prelude::{App, Plugin, Res};
use bevy_egui::{
    egui::{self, vec2, Align2, Window},
    EguiContexts,
};
use chrono::{TimeZone, Utc};

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
    let start_of_2000: i64 = 946684800000;

    if !game_resource.simulation.error_messages.is_empty() {
        Window::new("Error Book")
            .anchor(Align2::RIGHT_BOTTOM, vec2(0.0, 0.0))
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                egui::ScrollArea::vertical()
                    .id_source("errors_list")
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for message in &game_resource.simulation.error_messages {
                            if elapsed_time - message.0 < (8_000.0 * time_multiplier) as u128 {
                                let timestamp: i64 = start_of_2000 + message.0 as i64;
                                let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();
                                let text = format!(
                                    "{}, {}",
                                    datetime.format("%Y-%m-%d %H:%M").to_string(),
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
    let start_of_2000: i64 = 946684800000;

    if !game_resource.simulation.event_messages.is_empty() {
        Window::new("Log Book")
            .anchor(Align2::LEFT_BOTTOM, vec2(0.0, 0.0))
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                egui::ScrollArea::vertical()
                    .id_source("messages_list")
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for message in &game_resource.simulation.event_messages {
                            if elapsed_time - message.0 < (10_000.0 * time_multiplier) as u128 {
                                let timestamp: i64 = start_of_2000 + message.0 as i64;
                                let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();
                                let text = format!(
                                    "{}, {}",
                                    datetime.format("%Y-%m-%d %H:%M").to_string(),
                                    message.1
                                );
                                ui.label(text);
                            }
                        }
                    });
            });
    }
}
