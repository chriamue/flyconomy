use std::time::Duration;

use bevy::{
    prelude::{NextState, Query, ResMut},
    ui::Interaction,
};

use crate::{game::GameResource, simulation::DEFAULT_TIME_MULTIPLIER, ui::views::UiView};

use super::{
    AerodromesButton, AnalyticsButton, OfficeButton, PauseButton, PlayButton, ScheduleButton,
    SettingsButton, SimulationControl, SimulationControlAction, SkipButton, SpeedUpButton,
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

pub fn settings_button_system(
    mut interaction_query: Query<(&Interaction, &SettingsButton)>,
    mut ui_state_next_state: ResMut<NextState<UiView>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_state_next_state.set(UiView::Settings);
            }
            _ => {}
        }
    }
}

pub fn analytics_button_system(
    mut interaction_query: Query<(&Interaction, &AnalyticsButton)>,
    mut ui_state_next_state: ResMut<NextState<UiView>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_state_next_state.set(UiView::Analytics);
            }
            _ => {}
        }
    }
}

pub fn schedule_button_system(
    mut interaction_query: Query<(&Interaction, &ScheduleButton)>,
    mut ui_state_next_state: ResMut<NextState<UiView>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_state_next_state.set(UiView::Schedule);
            }
            _ => {}
        }
    }
}

pub fn aerodromes_button_system(
    mut interaction_query: Query<(&Interaction, &AerodromesButton)>,
    mut ui_state_next_state: ResMut<NextState<UiView>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_state_next_state.set(UiView::Aerodromes);
            }
            _ => {}
        }
    }
}

pub fn office_button_system(
    mut interaction_query: Query<(&Interaction, &OfficeButton)>,
    mut ui_state_next_state: ResMut<NextState<UiView>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_state_next_state.set(UiView::Office);
            }
            _ => {}
        }
    }
}
