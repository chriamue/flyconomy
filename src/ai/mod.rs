mod ai_action;
mod ai_agent;
mod ai_state;
mod replay_agent;

use std::time::Duration;

pub use ai_action::AiAction;
pub use ai_agent::AiAgent;
pub use ai_state::AiState;

use replay_agent::{ReplayAgent, ReplayStrategy, ReplayTerminationStrategy};

use rurel::{
    mdp::{Agent, State},
    strategy::{
        explore::{ExplorationStrategy, RandomExploration},
        learn::QLearning,
        terminate::FixedIterations,
    },
    AgentTrainer,
};

use crate::{
    config,
    model::{
        commands::{BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand},
        Aerodrome, Environment, PlaneType,
    },
    simulation::Simulation,
    Replay,
};

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

pub struct AiManager {
    pub trainer: AgentTrainer<AiState>,
    last_state_action: Option<(AiState, AiAction)>,
    no_op_counter: u32,
}

impl Default for AiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AiManager {
    pub fn new() -> Self {
        let trainer = AgentTrainer::new();
        Self {
            trainer,
            no_op_counter: 0,
            last_state_action: None,
        }
    }

    pub fn train(&mut self, iterations: u32) {
        let mut simulation = Simulation::new(
            Default::default(),
            config::aerodromes(),
            config::plane_types(),
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
            plane_type: simulation.plane_types[0].clone(),
            home_base_id: simulation.environment.bases[0].id,
        };

        simulation.add_command(Box::new(buy_plane_command));

        simulation.update(Duration::from_secs(1));

        // Start training

        let mut agent = AiAgent::new(&mut simulation);

        self.trainer.train(
            &mut agent,
            &QLearning::new(0.02, 0.05, 0.7),
            &mut FixedIterations::new(iterations),
            &RandomExploration::new(),
        );
        println!("{:?}", agent.state);
        println!("Planes: {:#?}", simulation.environment.planes);
        println!("Bases: {:#?}", simulation.environment.bases);
    }

    pub fn train_simulation(&mut self, simulation: &Simulation) {
        let replay = Replay::new(
            simulation.environment.config.clone(),
            simulation.command_history.clone(),
        );

        let mut simulation = Simulation::new(
            replay.initial_config.clone(),
            config::aerodromes(),
            config::plane_types(),
        );
        simulation.time_multiplier = 1.0;
        println!(
            "Replaying simulation {:?}",
            replay.command_history.last().unwrap().timestamp
        );

        let mut replay_agent = ReplayAgent::new(replay.clone(), &mut simulation);
        let replay_strategy = ReplayStrategy::new(replay.clone());
        let mut replay_termination_strategy = ReplayTerminationStrategy::new(replay);
        self.trainer.train(
            &mut replay_agent,
            &QLearning::new(0.02, 0.5, 0.7),
            &mut replay_termination_strategy,
            &replay_strategy,
        );
    }

    pub fn best_command(
        &mut self,
        environment: &Environment,
        plane_types: &Vec<PlaneType>,
        aerodromes: &Vec<Aerodrome>,
    ) -> Option<Box<dyn Command>> {
        let ai_state: AiState = environment.into();

        let mut action = self.trainer.best_action(&ai_state);

        if action.is_none() {
            self.no_op_counter += 1;
            if self.no_op_counter > 10 {
                action = Some(ai_state.random_action());
            }
        } else {
            self.no_op_counter = 0;
        }

        if let (Some(last_state_action), Some(cur_action)) = (&self.last_state_action, &action) {
            if last_state_action.1 == *cur_action {
                self.no_op_counter += 1;
                return None;
            }
        }

        action.as_ref().and_then(|action| {
            self.last_state_action = Some((ai_state, action.clone()));
            action.to_command(environment, aerodromes, plane_types)
        })
    }

    pub fn update(&mut self, simulation: &Simulation) {
        let mut agent = AiUpdateAgent::new(simulation);

        self.trainer.train(
            &mut agent,
            &QLearning::new(0.02, 0.01, 0.5),
            &mut FixedIterations::new(1),
            simulation,
        );
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ai_manager() {
        let mut ai_manager = super::AiManager::new();
        ai_manager.train(1000);
    }
}
