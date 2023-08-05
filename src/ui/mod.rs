use bevy::prelude::{App, Plugin};

pub mod components;

mod game_over_screen;
mod hud;
mod layouts;
mod messages;
mod simulation_control;
pub mod views;
mod welcome_screen;
mod world_heritage_site_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hud::HudPlugin);
        app.add_plugins(welcome_screen::WelcomeScreenPlugin);
        app.add_plugins(game_over_screen::GameOverScreenPlugin);
        app.add_plugins(world_heritage_site_ui::WorldHeritageSiteUiPlugin);
        app.add_plugins(messages::MessagesPlugin);
        app.add_plugins(simulation_control::SimulationControlPlugin);
        app.add_plugins(views::ViewsPlugin);
    }
}
