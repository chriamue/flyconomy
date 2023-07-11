use std::time::Duration;

use rurel::{
    mdp::{Agent, State},
    strategy::{explore::ExplorationStrategy, terminate::TerminationStrategy},
};

use crate::{model::Timestamp, simulation::Simulation, Replay};

use super::{AiAction, AiState};

pub struct ReplayAgent<'a> {
    replay: Replay,
    simulation: &'a mut Simulation,
    state: AiState,
    timestamp: Timestamp,
}

impl<'a> ReplayAgent<'a> {
    pub fn new(replay: Replay, simulation: &'a mut Simulation) -> Self {
        simulation.time_multiplier = 1.0;
        Self {
            state: (&simulation.environment).into(),
            replay,
            simulation,
            timestamp: 0,
        }
    }
}

impl<'a> Agent<AiState> for ReplayAgent<'a> {
    fn current_state(&self) -> &AiState {
        &self.state
    }

    fn take_action(&mut self, action: &<AiState as State>::A) {
        println!("take_action: {:?} at {:#?}", action, self.state);

        let command = self
            .replay
            .command_history
            .iter()
            .find(|command| command.timestamp == self.timestamp);
        let next_command = self
            .replay
            .command_history
            .iter()
            .find(|command| command.timestamp > self.timestamp);

        let delta_time = match next_command {
            Some(command) => command.timestamp - self.timestamp,
            None => 1,
        };

        println!("delta_time: {} of {}", delta_time, self.timestamp);

        match command {
            Some(command) => {
                self.simulation.add_command_timed(command.clone());
            }
            None => {}
        }
        self.simulation
            .update(Duration::from_millis(delta_time as u64));
        self.timestamp = self.simulation.environment.timestamp;
        self.state = (&self.simulation.environment).into();
    }
}

pub struct ReplayStrategy {
    replay: Replay,
}

impl ReplayStrategy {
    pub fn new(replay: Replay) -> Self {
        Self { replay }
    }
}

impl ExplorationStrategy<AiState> for ReplayStrategy {
    fn pick_action(&self, agent: &mut dyn Agent<AiState>) -> <AiState as State>::A {
        let timestamp = agent.current_state().timestamp;
        let action = self
            .replay
            .command_history
            .iter()
            .find(|action| action.timestamp >= timestamp);
        let action = match action {
            Some(action) => (&action.command).into(),
            None => AiAction::NoOp,
        };

        agent.take_action(&action);
        action
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        model::{
            commands::{
                BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand, ScheduleFlightCommand,
            },
            Aerodrome, StringBasedWorldData,
        },
        simulation::Simulation,
    };
    use rurel::{strategy::learn::QLearning, AgentTrainer};
    use std::time::Duration;

    #[test]
    fn test_replay_agent() {
        let mut simulation = Simulation::new(
            Default::default(),
            Box::new(StringBasedWorldData::default()),
        );
        simulation.setup();

        let paris_aerodrome = Aerodrome::new(
            1381,
            49.012798,
            2.55,
            "Paris, Charles de Gaulle".to_string(),
            "CDG/LFPG".to_string(),
        );

        let frankfurt_aerodrome = Aerodrome::new(
            339,
            50.033333,
            8.570556,
            "Frankfurt am Main Airport".to_string(),
            "FRA/EDDF".to_string(),
        );

        let create_base_command = CreateBaseCommand {
            base_id: CreateBaseCommand::generate_id(),
            aerodrome: frankfurt_aerodrome.clone(),
        };

        let buy_landing_rights_command = BuyLandingRightsCommand {
            landing_rights_id: BuyLandingRightsCommand::generate_id(),
            aerodrome: paris_aerodrome.clone(),
        };

        simulation.add_command(Box::new(create_base_command));
        simulation.update(Duration::from_secs(1));

        simulation.add_command(Box::new(buy_landing_rights_command));
        simulation.update(Duration::from_secs(1));

        let buy_plane_command = BuyPlaneCommand {
            plane_id: BuyPlaneCommand::generate_id(),
            plane_type: simulation.world_data_gateway.plane_types()[0].clone(),
            home_base_id: simulation.environment.bases[0].id,
        };

        simulation.add_command(Box::new(buy_plane_command));

        simulation.update(Duration::from_secs(1));

        assert_eq!(simulation.environment.planes.len(), 1);
        assert_eq!(simulation.environment.bases.len(), 1);
        assert_eq!(simulation.environment.landing_rights.len(), 1);
        assert_eq!(simulation.environment.flights.len(), 0);

        let flight_command = ScheduleFlightCommand {
            flight_id: ScheduleFlightCommand::generate_id(),
            airplane: simulation.environment.planes[0].clone(),
            origin_aerodrome: frankfurt_aerodrome.clone(),
            destination_aerodrome: paris_aerodrome.clone(),
            departure_time: (simulation.elapsed_time + Duration::from_secs(1)).as_millis(),
            interest_score: 0.0,
        };

        simulation.add_command(Box::new(flight_command));

        simulation.update(Duration::from_secs(1));

        let replay = Replay::new(
            simulation.environment.config.clone(),
            simulation.command_history.clone(),
        );

        let mut simulation = Simulation::new(
            replay.initial_config.clone(),
            Box::new(StringBasedWorldData::default()),
        );
        simulation.time_multiplier = 1.0;
        println!(
            "Replaying simulation {:?}",
            replay.command_history.last().unwrap().timestamp
        );

        let mut replay_agent = ReplayAgent::new(replay.clone(), &mut simulation);
        let replay_strategy = ReplayStrategy::new(replay.clone());
        let mut replay_termination_strategy = ReplayTerminationStrategy::new(replay);
        let mut trainer = AgentTrainer::new();
        trainer.train(
            &mut replay_agent,
            &QLearning::new(0.02, 0.5, 0.7),
            &mut replay_termination_strategy,
            &replay_strategy,
        );

        assert_eq!(simulation.environment.bases.len(), 1);
        assert_eq!(simulation.environment.landing_rights.len(), 1);
        assert_eq!(simulation.environment.planes.len(), 1);
        assert_eq!(simulation.environment.flights.len(), 1);
    }
}
