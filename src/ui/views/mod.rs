use bevy::prelude::{App, Plugin, States};

mod analytics_view;
mod office_view;
mod settings_view;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiView>();

        app.add_plugin(analytics_view::AnalyticsViewPlugin);
        app.add_plugin(office_view::OfficePlugin);
        app.add_plugin(settings_view::SettingsViewPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UiView {
    #[default]
    Aerodromes,
    Settings,
    Analytics,
    Schedule,
    Office,
}
