use bevy::prelude::{App, Plugin, States};

mod aerodromes_view;
mod analytics_view;
mod office_view;
mod schedule_view;
mod settings_view;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiView>();

        app.add_plugin(aerodromes_view::AerodromesUiPlugin);
        app.add_plugin(analytics_view::AnalyticsViewPlugin);
        app.add_plugin(schedule_view::ScheduleViewPlugin);
        app.add_plugin(office_view::OfficeViewPlugin);
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