use std::time::Duration;

use rurel::mdp::{Agent, State};

use crate::simulation::Simulation;

use super::AiState;

pub struct AiAgent<'a> {
    pub state: AiState,
    simulation: &'a mut Simulation,
}

impl<'a> AiAgent<'a> {
    pub fn new(simulation: &'a mut Simulation) -> Self {
        let environment = &simulation.environment;
        Self {
            state: environment.into(),
            simulation,
        }
    }

    fn update_state(&mut self) {
        let environment = &self.simulation.environment;
        self.state = environment.into();
    }
}

impl Agent<AiState> for AiAgent<'_> {
    fn current_state(&self) -> &AiState {
        &self.state
    }

    fn take_action(&mut self, action: &<AiState as State>::A) {
        let command = action.to_command(
            &self.simulation.environment,
            &self.simulation.aerodromes,
            &self.simulation.plane_types,
        );
        match command {
            Some(command) => self.simulation.add_command(command),
            None => {}
        }
        self.simulation.update(Duration::from_secs(1));
        self.update_state();
    }
}