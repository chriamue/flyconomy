use std::time::Duration;

use rurel::{
    mdp::State,
    strategy::{explore::RandomExploration, terminate::FixedIterations},
};

use crate::{
    ai::{AiAgent, GameOverTerminationStrategy, ReplayTerminationStrategy},
    model::{
        commands::{BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand},
        Aerodrome, Environment, PlaneType, StringBasedWorldData,
    },
    simulation::Simulation,
    Replay,
};

use super::{
    ai_trainer::AiTrainerType,
    replay_agent::{ReplayAgent, ReplayStrategy},
    AiAction, AiState, AiTrainer, AiUpdateAgent,
};

pub struct AiManager {
    pub trainer: Box<dyn AiTrainer>,
    last_state_action: Option<(AiState, AiAction)>,
    no_op_counter: u32,
}

unsafe impl Send for AiManager {}
unsafe impl Sync for AiManager {}

impl Default for AiManager {
    fn default() -> Self {
        Self::new(AiTrainerType::AgentTrainer)
    }
}

impl AiManager {
    pub fn new(trainer_type: AiTrainerType) -> Self {
        let trainer = trainer_type.create_trainer();
        Self {
            trainer,
            no_op_counter: 0,
            last_state_action: None,
        }
    }

    pub fn default_simulation() -> Simulation {
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
        simulation
    }

    pub fn train(&mut self, iterations: u32) {
        let mut simulation = Self::default_simulation();
        let environment = simulation.environment.clone();
        let mut agent = AiAgent::new(&mut simulation);

        let mut termination_strategy = GameOverTerminationStrategy::new(iterations, 120_000);
        while termination_strategy.i < iterations {
            simulation.elapsed_time = Duration::from_secs(0);
            simulation.command_history.clear();
            simulation.error_messages.clear();
            simulation.environment = environment.clone();
            agent = AiAgent::new(&mut simulation);
            self.trainer.train(
                &mut agent,
                &mut termination_strategy,
                &mut RandomExploration::new(),
            );
        }

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
            Box::new(StringBasedWorldData::default()),
        );
        simulation.time_multiplier = 25.0;
        println!(
            "Replaying simulation {:?}",
            replay.command_history.last().unwrap().timestamp
        );

        let mut replay_agent = ReplayAgent::new(replay.clone(), &mut simulation);
        let replay_strategy = ReplayStrategy::new(replay.clone());
        let mut replay_termination_strategy = ReplayTerminationStrategy::new(replay);
        self.trainer.train(
            &mut replay_agent,
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

        match &action {
            Some(cur_action) if *cur_action == AiAction::NoOp => self.no_op_counter += 1,
            Some(cur_action)
                if self
                    .last_state_action
                    .as_ref()
                    .map_or(false, |(_, a)| a == cur_action) =>
            {
                self.no_op_counter += 1
            }
            None => self.no_op_counter += 1,
            _ => self.no_op_counter = 0,
        }

        if self.no_op_counter > 10 {
            action = Some(ai_state.random_action());
            self.no_op_counter = 0; // Reset the counter after picking a random action
        }

        action.as_ref().and_then(|action| {
            self.last_state_action = Some((ai_state.clone(), action.clone()));
            let command = action.to_command(environment, aerodromes, plane_types);
            command
        })
    }

    pub fn update(&mut self, simulation: &Simulation) {
        let mut agent = AiUpdateAgent::new(simulation);

        self.trainer
            .train(&mut agent, &mut FixedIterations::new(0), simulation);
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::ai_trainer::AiTrainerType;

    #[test]
    fn test_ai_manager() {
        let mut ai_manager = super::AiManager::new(AiTrainerType::AgentTrainer);
        ai_manager.train(1000);
    }
}
