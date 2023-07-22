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
        app.add_plugin(hud::HudPlugin);
        app.add_plugin(welcome_screen::WelcomeScreenPlugin);
        app.add_plugin(game_over_screen::GameOverScreenPlugin);
        app.add_plugin(world_heritage_site_ui::WorldHeritageSiteUiPlugin);
        app.add_plugin(messages::MessagesPlugin);
        app.add_plugin(simulation_control::SimulationControlPlugin);
        app.add_plugin(views::ViewsPlugin);
    }
}
