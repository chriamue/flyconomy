use super::{layouts::left_layout, UiState};
use crate::{
    game::{GameResource, GameState},
    model::analytics::{
        calculate_average_profit_per_flight, calculate_cash_history,
        calculate_total_flight_distance, calculate_transported_passengers,
    },
};
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res};
use bevy_egui::{egui, EguiContexts};

pub struct AnalyticsViewPlugin;

impl Plugin for AnalyticsViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                company_hud_system,
                show_cash_history,
                show_total_flight_distance,
                show_transported_passengers,
                show_average_profit_per_flight,
            )
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Analytics)),
        );
    }
}

pub fn company_hud_system(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    left_layout("Company").show(contexts.ctx_mut(), |ui| {
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
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            cash_history_plot.show(ui, |ui| {
                ui.line(cash_history_line);
            });
        });
}

pub fn show_total_flight_distance(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let flight_distance_history =
        calculate_total_flight_distance(&game_resource.simulation.environment);

    let flight_distance_history_for_plot: Vec<[f64; 2]> = flight_distance_history
        .into_iter()
        .map(|(timestamp, distance)| {
            let timestamp = timestamp as f64;
            [timestamp, distance]
        })
        .collect();

    let flight_distance_history_line = egui::plot::Line::new(flight_distance_history_for_plot);

    let flight_distance_history_plot = egui::plot::Plot::new("Flight Distance History")
        .view_aspect(2.0)
        .label_formatter(|name, value| {
            if !name.is_empty() {
                format!("{}: {:.*} km", name, 1, value.y)
            } else {
                "".to_owned()
            }
        });

    egui::Window::new("Flight Distance History")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            flight_distance_history_plot.show(ui, |ui| {
                ui.line(flight_distance_history_line);
            });
        });
}

pub fn show_transported_passengers(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    let transported_passengers_history =
        calculate_transported_passengers(&game_resource.simulation.environment);

    let transported_passengers_history_for_plot: Vec<[f64; 2]> = transported_passengers_history
        .into_iter()
        .map(|(timestamp, passengers)| {
            let timestamp = timestamp as f64;
            [timestamp, passengers as f64]
        })
        .collect();

    let transported_passengers_history_line =
        egui::plot::Line::new(transported_passengers_history_for_plot);

    let transported_passengers_history_plot =
        egui::plot::Plot::new("Transported Passengers History")
            .view_aspect(2.0)
            .label_formatter(|name, value| {
                if !name.is_empty() {
                    format!("{}: {:.*} passengers", name, 1, value.y)
                } else {
                    "".to_owned()
                }
            });

    egui::Window::new("Transported Passengers History")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            transported_passengers_history_plot.show(ui, |ui| {
                ui.line(transported_passengers_history_line);
            });
        });
}

pub fn show_average_profit_per_flight(
    mut contexts: EguiContexts,
    game_resource: Res<GameResource>,
) {
    let average_profit_history =
        calculate_average_profit_per_flight(&game_resource.simulation.environment);

    let average_profit_history_for_plot: Vec<[f64; 2]> = average_profit_history
        .into_iter()
        .map(|(timestamp, average_profit)| {
            let timestamp = timestamp as f64;
            [timestamp, average_profit]
        })
        .collect();

    let average_profit_history_line = egui::plot::Line::new(average_profit_history_for_plot);

    let average_profit_history_plot = egui::plot::Plot::new("Average Profit Per Flight")
        .view_aspect(2.0)
        .label_formatter(|name, value| {
            if !name.is_empty() {
                format!("{}: $ {:.*}", name, 2, value.y)
            } else {
                "".to_owned()
            }
        });

    egui::Window::new("Average Profit Per Flight")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            average_profit_history_plot.show(ui, |ui| {
                ui.line(average_profit_history_line);
            });
        });
}
