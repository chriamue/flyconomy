use rurel::{
    dqn::DQNAgentTrainer,
    mdp::Agent,
    strategy::{explore::ExplorationStrategy, learn::QLearning, terminate::TerminationStrategy},
    AgentTrainer,
};

use super::{AiAction, AiState};

pub trait AiTrainer {
    fn train<'a>(
        &mut self,
        agent: &'a mut dyn Agent<AiState>,
        termination_strategy: &mut dyn TerminationStrategy<AiState>,
        exploration_strategy: &dyn ExplorationStrategy<AiState>,
    );
    fn best_action(&self, state: &AiState) -> Option<AiAction>;
}

pub enum AiTrainerType {
    AgentTrainer,
    DQNAgentTrainer,
}

impl AiTrainerType {
    pub fn create_trainer(&self) -> Box<dyn AiTrainer> {
        match self {
            Self::AgentTrainer => {
                let trainer: AgentTrainer<AiState> = AgentTrainer::new();
                Box::new(trainer)
            }
            Self::DQNAgentTrainer => {
                let trainer: DQNAgentTrainer<AiState, 7, 8, 64> = DQNAgentTrainer::new(0.7, 1e-3);
                Box::new(trainer)
            }
        }
    }
}

impl AiTrainer for AgentTrainer<AiState> {
    fn train<'a>(
        &mut self,
        agent: &'a mut dyn Agent<AiState>,
        termination_strategy: &mut dyn TerminationStrategy<AiState>,
        exploration_strategy: &dyn ExplorationStrategy<AiState>,
    ) {
        AgentTrainer::train(
            self,
            agent,
            &QLearning::new(0.02, 0.5, 0.7),
            termination_strategy,
            exploration_strategy,
        );
    }

    fn best_action(&self, state: &AiState) -> Option<AiAction> {
        AgentTrainer::best_action(self, &state)
    }
}

impl AiTrainer for DQNAgentTrainer<AiState, 7, 8, 64> {
    fn train<'a>(
        &mut self,
        agent: &'a mut dyn Agent<AiState>,
        termination_strategy: &mut dyn TerminationStrategy<AiState>,
        exploration_strategy: &dyn ExplorationStrategy<AiState>,
    ) {
        DQNAgentTrainer::train(self, agent, termination_strategy, exploration_strategy);
    }

    fn best_action(&self, state: &AiState) -> Option<AiAction> {
        DQNAgentTrainer::best_action(self, &state)
    }
}
