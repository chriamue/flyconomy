use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, ResMut, Resource, States};
use bevy_egui::{
    egui::{vec2, Align2, Visuals, Window},
    EguiContexts,
};

use crate::game::GameState;

mod aerodromes_ui;
mod analytics_ui;
mod flights_ui;
mod game_over_screen;
mod hud;
mod messages;
mod office_ui;
mod planes_ui;
mod player_ownership_ui;
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
        app.add_plugin(player_ownership_ui::PlayerOwnershipUiPlugin);
        app.add_plugin(flights_ui::FlightsUiPlugin);
        app.add_plugin(replay::ReplayPlugin);
        app.add_plugin(simulation_control::SimulationControlPlugin);
        app.add_plugin(analytics_ui::AnalyticsPlugin);
        app.add_plugin(office_ui::OfficePlugin);
        app.insert_resource(StyleState { is_dark: true });
        app.add_systems(
            (style_switch_ui,)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Settings)),
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

#[derive(Default, Resource)]
pub struct StyleState {
    pub is_dark: bool,
}

fn style_switch_ui(mut contexts: EguiContexts, mut style_state: ResMut<StyleState>) {
    Window::new("Style Switch")
        .anchor(Align2::LEFT_TOP, vec2(0.0, 100.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let mut is_dark = style_state.is_dark;
            if ui.checkbox(&mut is_dark, "Dark Mode").clicked() {
                style_state.is_dark = is_dark;
                if is_dark {
                    ui.ctx().set_visuals(Visuals::dark());
                } else {
                    ui.ctx().set_visuals(Visuals::light());
                }
            }
        });
}
