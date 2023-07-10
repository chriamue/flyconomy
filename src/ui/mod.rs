use bevy::prelude::{App, EventWriter, OnUpdate, Query, Res, States, Transform};
use bevy::prelude::{IntoSystemConfigs, Plugin};
use bevy_egui::egui::{vec2, Align2};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::game::aerodrome::SelectedAerodromeChangeEvent;
use crate::game::{GameResource, GameState};

mod aerodromes_ui;
mod analytics_ui;
mod flights_ui;
mod game_over_screen;
mod hud;
mod messages;
mod office_ui;
mod planes_ui;
mod replay;
mod simulation_control;
mod welcome_screen;
mod world_heritage_site_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiState>();
        app.add_plugin(hud::HudPlugin);
        app.add_plugin(welcome_screen::WelcomeScreenPlugin);
        app.add_plugin(game_over_screen::GameOverScreenPlugin);
        app.add_plugin(aerodromes_ui::AerodromesUiPlugin);
        app.add_plugin(world_heritage_site_ui::WorldHeritageSiteUiPlugin);
        app.add_plugin(messages::MessagesPlugin);
        app.add_plugin(planes_ui::PlanesUiPlugin);
        app.add_plugin(flights_ui::FlightsUiPlugin);
        app.add_plugin(replay::ReplayPlugin);
        app.add_plugin(simulation_control::SimulationControlPlugin);
        app.add_plugin(analytics_ui::AnalyticsPlugin);
        app.add_plugin(office_ui::OfficePlugin);
        app.add_systems(
            (bases_info_ui, landing_rights_info_ui)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Aerodromes)),
        );
        app.add_systems(
            (bases_info_ui, landing_rights_info_ui)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Schedule)),
        );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UiState {
    #[default]
    Aerodromes,
    Settings,
    Analytics,
    Schedule,
    Office,
}

pub fn bases_info_ui(
    mut contexts: EguiContexts,
    game_resource: Res<GameResource>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    egui::Window::new("Bases Info")
        .anchor(Align2::RIGHT_TOP, vec2(0.0, 100.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let environment = &game_resource.simulation.environment;
            ui.label("Owned Bases:");

            for base in &environment.bases {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(false, format!("Aerodrome: {}", base.aerodrome.name))
                        .clicked()
                    {
                        ev_selected_aerodrome_change
                            .send(SelectedAerodromeChangeEvent(base.aerodrome.clone()));
                        let alpha = (90.0 + base.aerodrome.lon).to_radians();
                        let beta = base.aerodrome.lat.to_radians();
                        for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                            pan_orbit.target_alpha = alpha as f32;
                            pan_orbit.target_beta = beta as f32;
                            pan_orbit.radius = Some(1.5);
                            pan_orbit.force_update = true;
                        }
                    }
                    ui.label(format!("Number of Airplanes: {}", base.airplane_ids.len()));
                });
            }
        });
}

pub fn landing_rights_info_ui(
    mut contexts: EguiContexts,
    game_resource: Res<GameResource>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    egui::Window::new("Landing Rights Info")
        .anchor(Align2::RIGHT_TOP, vec2(0.0, 300.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let environment = &game_resource.simulation.environment;
            ui.label("Owned Landing Rights:");

            for landing_rights in &environment.landing_rights {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            false,
                            format!("Aerodrome: {}", landing_rights.aerodrome.name),
                        )
                        .clicked()
                    {
                        ev_selected_aerodrome_change.send(SelectedAerodromeChangeEvent(
                            landing_rights.aerodrome.clone(),
                        ));
                        let alpha = (90.0 + landing_rights.aerodrome.lon).to_radians();
                        let beta = landing_rights.aerodrome.lat.to_radians();
                        for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                            pan_orbit.target_alpha = alpha as f32;
                            pan_orbit.target_beta = beta as f32;
                            pan_orbit.radius = Some(1.5);
                            pan_orbit.force_update = true;
                        }
                    }
                });
            }
        });
}
