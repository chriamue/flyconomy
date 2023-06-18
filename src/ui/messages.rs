use crate::game::GameResource;
use bevy::prelude::{App, Plugin, Res};
use bevy_egui::{egui, EguiContexts};

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_error_messages);
    }
}

pub fn show_error_messages(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let elapsed_time = game_resource.simulation.elapsed_time.as_secs_f64();
    let time_multiplier = game_resource.simulation.time_multiplier;

    if !game_resource.simulation.error_messages.is_empty() {
        egui::Window::new("Log Book")
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                for message in &game_resource.simulation.error_messages {
                    if elapsed_time - message.0 < 5.0 * time_multiplier {
                        let text = format!("{:.0}, {}", message.0, message.1);
                        ui.label(text);
                    }
                }
            });
    }
}
