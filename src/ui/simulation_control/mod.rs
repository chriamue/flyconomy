use crate::game::GameState;
use bevy::prelude::*;

mod actions;
mod layout;
mod styles;

use actions::{
    aerodromes_button_system, analytics_button_system, pause_button_system, play_button_system,
    schedule_button_system, settings_button_system, skip_button_system, speed_up_button_system,
};
use layout::{despawn_simulation_control_buttons, spawn_simulation_control_buttons};

use self::actions::office_button_system;

#[derive(Default, Resource)]
pub struct SimulationControl {
    pub action: SimulationControlAction,
}

#[derive(Default)]
pub enum SimulationControlAction {
    #[default]
    Play,
    SpeedUp,
    SkipToEnd,
    Pause,
}

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SpeedUpButton;

#[derive(Component)]
pub struct SkipButton;

#[derive(Component)]
pub struct PauseButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct AnalyticsButton;

#[derive(Component)]
pub struct ScheduleButton;

#[derive(Component)]
pub struct AerodromesButton;

#[derive(Component)]
pub struct OfficeButton;

pub struct SimulationControlPlugin;

impl Plugin for SimulationControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_simulation_control_buttons,),
        )
        .insert_resource(SimulationControl::default())
        .add_systems(
            Update,
            (
                play_button_system,
                speed_up_button_system,
                skip_button_system,
                pause_button_system,
                settings_button_system,
                analytics_button_system,
                schedule_button_system,
                aerodromes_button_system,
                office_button_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (despawn_simulation_control_buttons,),
        );
    }
}
