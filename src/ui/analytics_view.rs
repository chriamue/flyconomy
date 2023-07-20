use super::{
    components::analytics::{
        average_profit_per_flight, cash_history, company_finances, total_flight_distance,
        transported_passengers,
    },
    layouts::{left_layout, right_layout},
    UiState,
};
use crate::game::{GameResource, GameState};
use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res};
use bevy_egui::{egui, EguiContexts};

pub struct AnalyticsViewPlugin;

impl Plugin for AnalyticsViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (company_info_system, flight_analytics_system)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Analytics)),
        );
    }
}

pub fn company_info_system(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    left_layout("Company").show(contexts.ctx_mut(), |ui| {
        company_finances(ui, &game_resource);

        egui::CollapsingHeader::new("Cash History")
            .default_open(true)
            .show(ui, |ui| {
                cash_history(ui, &game_resource);
            });
        egui::CollapsingHeader::new("Average Profit Per Flight")
            .default_open(true)
            .show(ui, |ui| {
                average_profit_per_flight(ui, &game_resource);
            });
    });
}

pub fn flight_analytics_system(mut contexts: EguiContexts, game_resource: Res<GameResource>) {
    right_layout("Flight Analytics").show(contexts.ctx_mut(), |ui| {
        egui::CollapsingHeader::new("Total Flight Distance")
            .default_open(true)
            .show(ui, |ui| {
                total_flight_distance(ui, &game_resource);
            });
        egui::CollapsingHeader::new("Transported Passengers")
            .default_open(true)
            .show(ui, |ui| {
                transported_passengers(ui, &game_resource);
            });
    });
}
