use crate::game::GameState;
use bevy::prelude::*;

mod actions;
mod layout;
mod styles;

use actions::{
    pause_button_system, play_button_system, skip_button_system, speed_up_button_system,
};
use layout::{despawn_simulation_control_buttons, spawn_simulation_control_buttons};

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

pub struct SimulationControlPlugin;

impl Plugin for SimulationControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_simulation_control_buttons.in_schedule(OnEnter(GameState::Playing)))
            .insert_resource(SimulationControl::default())
            .add_systems(
                (
                    play_button_system,
                    speed_up_button_system,
                    skip_button_system,
                    pause_button_system,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(despawn_simulation_control_buttons.in_schedule(OnExit(GameState::Playing)));
    }
}
