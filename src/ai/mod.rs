mod ai_action;
mod ai_agent;
mod ai_manager;
mod ai_state;
mod ai_trainer;
mod replay_agent;
mod termination_strategies;

pub use ai_action::AiAction;
pub use ai_agent::AiAgent;
pub use ai_manager::AiManager;
pub use ai_state::AiState;
pub use ai_trainer::{AiTrainer, AiTrainerType};
pub use termination_strategies::{GameOverTerminationStrategy, ReplayTerminationStrategy};

use rurel::{
    mdp::{Agent, State},
    strategy::explore::ExplorationStrategy,
};

use crate::simulation::Simulation;

struct AiUpdateAgent {
    state: AiState,
}

impl AiUpdateAgent {
    pub fn new(simulation: &Simulation) -> Self {
        Self {
            state: (&simulation.environment).into(),
        }
    }
}

impl Agent<AiState> for AiUpdateAgent {
    fn current_state(&self) -> &AiState {
        &self.state
    }

    fn take_action(&mut self, _action: &<AiState as State>::A) {}
}

impl ExplorationStrategy<AiState> for Simulation {
    fn pick_action(&self, _: &mut dyn Agent<AiState>) -> <AiState as State>::A {
        (&self.command_history.last().unwrap().clone().command).into()
    }
}
