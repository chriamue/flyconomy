use bevy::prelude::{App, Plugin, States};

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
