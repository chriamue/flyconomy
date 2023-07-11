use super::GameResource;
use crate::ai::{AiManager, AiTrainerType};
use bevy::prelude::Time;
use bevy::prelude::{App, Plugin, Res, ResMut, Resource};
use bevy::time::Timer;
use std::sync::atomic::{AtomicUsize, Ordering};
use strum::EnumIter;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameManagers::default());
        app.add_system(manager_action_system);
    }
}

static MANAGER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, EnumIter)]
pub enum GameManagerType {
    QManager1,
    QManager2,
    DQNManger,
}

impl GameManagerType {
    pub fn create_manager(&self) -> GameManager {
        let mut manager = match self {
            Self::QManager1 => GameManager {
                id: MANAGER_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                ai_manager: AiManager::new(AiTrainerType::AgentTrainer),
                is_working: false,
                manager_action: "".to_string(),
                timer: Timer::from_seconds(2.5, bevy::time::TimerMode::Repeating),
            },
            Self::QManager2 => GameManager {
                id: MANAGER_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                ai_manager: AiManager::new(AiTrainerType::AgentTrainer),
                is_working: false,
                manager_action: "".to_string(),
                timer: Timer::from_seconds(0.5, bevy::time::TimerMode::Repeating),
            },
            Self::DQNManger => GameManager {
                id: MANAGER_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                ai_manager: AiManager::new(AiTrainerType::DQNAgentTrainer),
                is_working: false,
                manager_action: "".to_string(),
                timer: Timer::from_seconds(1.0, bevy::time::TimerMode::Repeating),
            },
        };
        for _epoch in 0..5 {
            manager.ai_manager.train(2500);
        }
        manager
    }
}

#[derive(Resource, Default)]
pub struct GameManagers {
    pub managers: Vec<GameManager>,
}

pub struct GameManager {
    pub id: usize,
    pub ai_manager: AiManager,
    pub is_working: bool,
    pub manager_action: String,
    pub timer: Timer,
}

pub fn manager_action_system(
    time: Res<Time>,
    mut game_resource: ResMut<GameResource>,
    mut game_managers: ResMut<GameManagers>,
) {
    for manager in &mut game_managers.managers {
        manager.timer.tick(time.delta());

        if manager.timer.finished() && manager.is_working {
            let environment = &game_resource.simulation.environment;

            let world_data_gateway = &game_resource.simulation.world_data_gateway;
            let command = manager.ai_manager.best_command(
                environment,
                world_data_gateway.plane_types(),
                world_data_gateway.aerodromes(),
                world_data_gateway.world_heritage_sites(),
            );
            if let Some(command) = command {
                manager.manager_action = format!("{:#?}", command);
                game_resource.simulation.add_command(command);
            }
        }
    }
}
