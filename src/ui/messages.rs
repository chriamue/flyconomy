use crate::game::GameResource;
use bevy::prelude::{App, Plugin, Res};
use bevy_egui::{egui, EguiContexts};

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_error_messages);
    }
}
use std::collections::VecDeque;

pub fn show_error_messages(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let elapsed_time = game_resource.simulation.elapsed_time.as_secs_f64();
    let time_multiplier = game_resource.simulation.time_multiplier;

    let mut relevant_messages = VecDeque::new();

    for message in game_resource.simulation.error_messages.iter().rev() {
        let message_timestamp = message.0;
        if elapsed_time - message_timestamp < 5.0 * time_multiplier {
            relevant_messages.push_front(message);
        }
    }

    if !relevant_messages.is_empty() {
        egui::Window::new("Error Messages")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                for message in &relevant_messages {
                    let text = format!("Timestamp: {:.2}, Error: {}", message.0, message.1);
                    ui.label(text);
                }
            });
    }
}
