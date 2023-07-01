use super::{
    styles::IMAGE_STYLE, AerodromesButton, AnalyticsButton, PauseButton, PlayButton,
    ScheduleButton, SettingsButton, SkipButton, SpeedUpButton,
};
use bevy::prelude::*;

use super::styles::{ITEM_STYLE, SIMULATION_CONTROL_STYLE};

pub fn spawn_simulation_control_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_simulation_control(&mut commands, &asset_server);
}

pub fn build_simulation_control(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let control_entity = commands
        .spawn(NodeBundle {
            style: SIMULATION_CONTROL_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            spawn_control_button(parent, "icons/pause-button.png", &asset_server, PauseButton);
            spawn_control_button(parent, "icons/play-button.png", &asset_server, PlayButton);
            spawn_control_button(
                parent,
                "icons/fast-forward-button.png",
                &asset_server,
                SpeedUpButton,
            );
            spawn_control_button(parent, "icons/next-button.png", &asset_server, SkipButton);
            spawn_control_button(parent, "icons/cog.png", &asset_server, SettingsButton);
            spawn_control_button(parent, "icons/diagram.png", &asset_server, AnalyticsButton);
            spawn_control_button(
                parent,
                "icons/control-tower.png",
                &asset_server,
                AerodromesButton,
            );
            spawn_control_button(
                parent,
                "icons/plane-pilot.png",
                &asset_server,
                ScheduleButton,
            );
        })
        .id();

    control_entity
}

fn spawn_control_button(
    parent: &mut ChildBuilder,
    icon_path: &str,
    asset_server: &Res<AssetServer>,
    button_component: impl Component,
) {
    parent
        .spawn(ButtonBundle {
            style: ITEM_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load(icon_path).into(),
                ..Default::default()
            });
        })
        .insert(button_component);
}

pub fn despawn_simulation_control_buttons(
    mut commands: Commands,
    button_query: Query<
        Entity,
        Or<(
            With<PlayButton>,
            With<SpeedUpButton>,
            With<SkipButton>,
            With<PauseButton>,
            With<SettingsButton>,
        )>,
    >,
) {
    for entity in button_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
