use std::time::Duration;

use bevy::{
    prelude::{Query, ResMut},
    ui::Interaction,
};

use crate::{game::GameResource, simulation::DEFAULT_TIME_MULTIPLIER};

use super::{
    PauseButton, PlayButton, SimulationControl, SimulationControlAction, SkipButton, SpeedUpButton,
};

pub fn play_button_system(
    mut game_resource: ResMut<GameResource>,
    mut simulation_control: ResMut<SimulationControl>,
    mut interaction_query: Query<(&Interaction, &PlayButton)>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_resource.simulation.time_multiplier = DEFAULT_TIME_MULTIPLIER;
                simulation_control.action = SimulationControlAction::Play;
            }
            _ => {}
        }
    }
}

pub fn speed_up_button_system(
    mut game_resource: ResMut<GameResource>,
    mut simulation_control: ResMut<SimulationControl>,
    mut interaction_query: Query<(&Interaction, &SpeedUpButton)>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_resource.simulation.time_multiplier *= 1.5;
                simulation_control.action = SimulationControlAction::SpeedUp;
            }
            _ => {}
        }
    }
}

pub fn skip_button_system(
    mut game_resource: ResMut<GameResource>,
    mut simulation_control: ResMut<SimulationControl>,
    mut interaction_query: Query<(&Interaction, &SkipButton)>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_resource.simulation.time_multiplier = DEFAULT_TIME_MULTIPLIER;

                while game_resource.simulation.commands.len() > 0 {
                    game_resource.simulation.update(Duration::from_millis(20));
                }

                simulation_control.action = SimulationControlAction::SkipToEnd;
            }
            _ => {}
        }
    }
}

pub fn pause_button_system(
    mut game_resource: ResMut<GameResource>,
    mut simulation_control: ResMut<SimulationControl>,
    mut interaction_query: Query<(&Interaction, &PauseButton)>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_resource.simulation.time_multiplier = 0.0;
                simulation_control.action = SimulationControlAction::Pause;
            }
            _ => {}
        }
    }
}
