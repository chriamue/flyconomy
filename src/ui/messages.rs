use crate::game::GameResource;
use bevy::prelude::{App, Plugin, Res};
use bevy_egui::{egui::Window, EguiContexts};

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
        Window::new("Error Book")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                for message in &game_resource.simulation.error_messages {
                    if elapsed_time - message.0 < (8_000.0 * time_multiplier) as u128 {
                        let text = format!("{:.0}, {}", message.0, message.1);
                        ui.label(text);
                    }
                }
            });
    }
}

pub fn show_event_messages(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let elapsed_time = game_resource.simulation.elapsed_time.as_millis();
    let time_multiplier = game_resource.simulation.time_multiplier;

    if !game_resource.simulation.event_messages.is_empty() {
        Window::new("Log Book")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                for message in &game_resource.simulation.event_messages {
                    if elapsed_time - message.0 < (10_000.0 * time_multiplier) as u128 {
                        let text = format!("{:.0}, {}", message.0, message.1);
                        ui.label(text);
                    }
                }
            });
    }
}
