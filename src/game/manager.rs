use bevy::prelude::{App, Plugin, Res, ResMut, Resource};

use crate::ai::AiManager;
use bevy::prelude::Time;
use bevy::time::Timer;

use super::{ConfigResource, GameResource};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        let mut manager_action = ManagerAction::default();
        manager_action.ai_manager.train(3_000);
        println!(
            "Manager Action: {:?}",
            manager_action.ai_manager.trainer.learned_values()
        );
        app.insert_resource(ManagerTimer::default());
        app.insert_resource(manager_action);
        app.add_system(manager_action_system);
    }
}

#[derive(Resource, Default)]
pub struct ManagerAction {
    pub manager_action: String,
    pub is_working: bool,
    pub ai_manager: AiManager,
}

#[derive(Resource)]
pub struct ManagerTimer {
    pub timer: Timer,
}

impl Default for ManagerTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, bevy::time::TimerMode::Repeating),
        }
    }
}

pub fn manager_action_system(
    time: Res<Time>,
    mut manager_timer: ResMut<ManagerTimer>,
    config_resource: Res<ConfigResource>,
    mut game_resource: ResMut<GameResource>,
    mut manager_action: ResMut<ManagerAction>,
) {
    manager_timer.timer.tick(time.delta());

    if manager_timer.timer.finished() && manager_action.is_working {
        let environment = &game_resource.simulation.environment;

        if let (Some(plane_types), Some(aerodromes)) =
            (&config_resource.planes_config, &config_resource.aerodromes)
        {
            let command = manager_action.ai_manager.best_command(
                environment,
                &plane_types.planes,
                aerodromes,
            );
            if let Some(command) = command {
                manager_action.manager_action = format!("{:#?}", command);
                game_resource.simulation.add_command(command);
            }
        }
    }
}
