use crate::{
    game::{GameResource, GameState},
    model::analytics::calculate_cash_history,
};
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res};
use bevy_egui::{egui, EguiContexts};

pub struct AnalyticsPlugin;

impl Plugin for AnalyticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((show_cash_history,).in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn show_cash_history(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let cash_history = calculate_cash_history(&game_resource.simulation.environment);

    let cash_history_for_plot: Vec<[f64; 2]> = cash_history
        .into_iter()
        .map(|(timestamp, cash)| {
            let timestamp = timestamp as f64;
            [timestamp, cash]
        })
        .collect();

    let cash_history_line = egui::plot::Line::new(cash_history_for_plot);

    let cash_history_plot = egui::plot::Plot::new("Cash History")
        .view_aspect(2.0)
        .label_formatter(|name, value| {
            if !name.is_empty() {
                format!("{}: {:.*}%", name, 1, value.y)
            } else {
                "".to_owned()
            }
        });

    egui::Window::new("Cash History")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            cash_history_plot.show(ui, |ui| {
                ui.line(cash_history_line);
            });
        });
}
