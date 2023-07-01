use crate::{
    game::{GameResource, GameState},
    model::analytics::calculate_cash_history,
};
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res};
use bevy_egui::{
    egui::{self, vec2, Align2},
    EguiContexts,
};

use super::UiState;

pub struct AnalyticsPlugin;

impl Plugin for AnalyticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (company_hud_system, show_cash_history)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Analytics)),
        );
    }
}

pub fn company_hud_system(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    egui::Window::new("Company")
        .anchor(Align2::LEFT_TOP, vec2(0.0, 100.0))
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            let environment = &game_resource.simulation.environment;
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Cash: ${:.2}",
                    environment.company_finances.cash(environment.timestamp)
                ));
                ui.label(format!("Planes: {}", environment.planes.len()));
                ui.label(format!(
                    "Total Income: ${:.2}",
                    environment
                        .company_finances
                        .total_income(environment.timestamp)
                ));
                ui.label(format!(
                    "Total Expenses: ${:.2}",
                    environment
                        .company_finances
                        .total_expenses(environment.timestamp)
                ));
            });
        });
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
        .anchor(egui::Align2::LEFT_CENTER, vec2(0.0, 0.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            cash_history_plot.show(ui, |ui| {
                ui.line(cash_history_line);
            });
        });
}
