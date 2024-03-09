use bevy::prelude::{App, Plugin, States};

mod aerodromes_view;
mod analytics_view;
mod office_view;
mod schedule_view;
mod settings_view;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiView>();

        app.add_plugins(aerodromes_view::AerodromesUiPlugin);
        app.add_plugins(analytics_view::AnalyticsViewPlugin);
        app.add_plugins(schedule_view::ScheduleViewPlugin);
        app.add_plugins(office_view::OfficeViewPlugin);
        app.add_plugins(settings_view::SettingsViewPlugin);
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
