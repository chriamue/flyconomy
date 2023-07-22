use rurel::strategy::terminate::TerminationStrategy;

use crate::Replay;

use super::AiState;

pub struct ReplayTerminationStrategy {
    replay: Replay,
}

impl ReplayTerminationStrategy {
    pub fn new(replay: Replay) -> Self {
        Self { replay }
    }
}

impl TerminationStrategy<AiState> for ReplayTerminationStrategy {
    fn should_stop(&mut self, state: &AiState) -> bool {
        self.replay
            .command_history
            .iter()
            .find(|command| command.timestamp >= state.timestamp)
            .is_none()
    }
}

pub struct GameOverTerminationStrategy {
    pub i: u32,
    pub iters: u32,
    min_cash: u64,
}

impl GameOverTerminationStrategy {
    pub fn new(iters: u32, min_cash: u64) -> Self {
        Self {
            i: 0,
            iters,
            min_cash,
        }
    }
}

impl TerminationStrategy<AiState> for GameOverTerminationStrategy {
    fn should_stop(&mut self, state: &AiState) -> bool {
        self.i += 1;
        self.i >= self.iters || state.cash < self.min_cash
    }
}
