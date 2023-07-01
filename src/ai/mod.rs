mod ai_action;
mod ai_agent;
mod ai_state;

use std::time::Duration;

pub use ai_action::AiAction;
pub use ai_agent::AiAgent;
pub use ai_state::AiState;
use rand::seq::SliceRandom;
use rurel::{
    mdp::State,
    strategy::{explore::RandomExploration, learn::QLearning, terminate::FixedIterations},
    AgentTrainer,
};

use crate::{
    config::{load_airports, PlanesConfig},
    model::{
        commands::{BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand},
        Aerodrome, Environment, PlaneType,
    },
    simulation::Simulation,
};

pub struct AiManager {
    pub trainer: AgentTrainer<AiState>,
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
        }
    }

    pub fn train(&mut self, iterations: u32) {
        let aerodromes = load_airports(
            include_str!("../../assets/airports.dat"),
            include_str!("../../assets/passengers.csv"),
        );

        let planes_config: PlanesConfig =
            serde_yaml::from_str(include_str!("../../assets/planes.yaml")).unwrap();

        let mut simulation = Simulation::new(Default::default());
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
            aerodrome: frankfurt_aerodrome.clone(),
        };

        let buy_landing_rights_command = BuyLandingRightsCommand {
            aerodrome: paris_aerodrome.clone(),
        };

        simulation.add_command(Box::new(create_base_command));
        simulation.update(Duration::from_secs(1));

        simulation.add_command(Box::new(buy_landing_rights_command));
        simulation.update(Duration::from_secs(1));

        let buy_plane_command = BuyPlaneCommand {
            plane_type: planes_config.planes[0].clone(),
            home_base_id: simulation.environment.bases[0].id,
        };

        simulation.add_command(Box::new(buy_plane_command));

        simulation.update(Duration::from_secs(1));

        // Start training

        let mut agent = AiAgent::new(&mut simulation, planes_config.planes, aerodromes);

        self.trainer.train(
            &mut agent,
            &QLearning::new(0.2, 0.01, 2.0),
            &mut FixedIterations::new(iterations),
            &RandomExploration::new(),
        );
        println!("{:?}", agent.state);
        println!("Planes: {:#?}", simulation.environment.planes);
        println!("Bases: {:#?}", simulation.environment.bases);
    }

    pub fn best_command(
        &mut self,
        environment: &Environment,
        plane_types: &Vec<PlaneType>,
        aerodromes: &Vec<Aerodrome>,
    ) -> Option<Box<dyn Command>> {
        let ai_state: AiState = environment.into();
        let action = self.trainer.best_action(&ai_state);

        let action = if let None = action {
            self.no_op_counter += 1;
            if self.no_op_counter > 100 {
                let actions = ai_state.actions();
                self.no_op_counter = 0;
                actions.choose(&mut rand::thread_rng()).cloned()
            } else {
                None
            }
        } else {
            self.no_op_counter = 0;
            action
        };
        match action {
            Some(action) => {
                let command = action.to_command(environment, plane_types, aerodromes);
                match command {
                    Some(command) => Some(command),
                    None => None,
                }
            }
            None => None,
        }
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
