use std::time::Duration;

use rurel::mdp::{Agent, State};

use crate::{
    model::{
        commands::{
            BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand,
            ScheduleFlightCommand,
        },
        Aerodrome, Environment, PlaneType,
    },
    simulation::Simulation,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AiAction {
    NoOp,
    BuyPlane {
        plane_type: u8,
        base_id: u64,
    },
    CreateBase {
        aerodrome_id: u64,
    },
    BuyLandingRights {
        aerodrome_id: u64,
    },
    ScheduleFlight {
        plane_id: u64,
        origin_id: u64,
        destination_id: u64,
    },
}

impl AiAction {
    pub fn to_command(
        &self,
        environment: &Environment,
        plane_types: &Vec<PlaneType>,
        aerodromes: &Vec<Aerodrome>,
    ) -> Option<Box<dyn Command>> {
        match self {
            AiAction::BuyPlane {
                plane_type,
                base_id,
            } => Some(Box::new(BuyPlaneCommand {
                plane_type: plane_types[*plane_type as usize].clone(),
                home_base_id: *base_id,
            })),
            AiAction::CreateBase { aerodrome_id } => {
                match aerodromes
                    .iter()
                    .find(|aerodrome| aerodrome.id == *aerodrome_id)
                {
                    Some(aerodrome) => Some(Box::new(CreateBaseCommand {
                        aerodrome: aerodrome.clone(),
                    })),
                    None => return None,
                }
            }
            AiAction::BuyLandingRights { aerodrome_id } => {
                match aerodromes
                    .iter()
                    .find(|aerodrome| aerodrome.id == *aerodrome_id)
                {
                    Some(aerodrome) => Some(Box::new(BuyLandingRightsCommand {
                        aerodrome: aerodrome.clone(),
                    })),
                    None => return None,
                }
            }
            AiAction::ScheduleFlight {
                plane_id,
                origin_id,
                destination_id,
            } => {
                let airplane = environment
                    .planes
                    .iter()
                    .find(|plane| plane.id == *plane_id)
                    .unwrap()
                    .clone();

                let origin_aerodrome: Aerodrome = environment
                    .bases
                    .iter()
                    .find(|base| base.id == *origin_id)
                    .unwrap()
                    .aerodrome
                    .clone();

                let destination_aerodrome: Aerodrome = environment
                    .landing_rights
                    .iter()
                    .find(|landing_rights| landing_rights.aerodrome.id == *destination_id)
                    .unwrap()
                    .aerodrome
                    .clone();

                Some(Box::new(ScheduleFlightCommand {
                    airplane,
                    origin_aerodrome,
                    destination_aerodrome,
                    departure_time: environment.timestamp,
                }))
            }
            AiAction::NoOp => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct AiState {
    cash: u64,
    planes: Vec<u64>,
    bases: Vec<u64>,
    landing_rights: Vec<u64>,
}

impl State for AiState {
    type A = AiAction;

    fn reward(&self) -> f64 {
        self.cash as f64
    }

    fn actions(&self) -> Vec<Self::A> {
        let mut actions = vec![AiAction::NoOp];
        if self.cash >= 350_000 {
            for base_id in &self.bases {
                for plane_type in 0..3 {
                    actions.push(AiAction::BuyPlane {
                        plane_type,
                        base_id: *base_id,
                    });
                }
            }
        }
        if self.cash >= 800_000 {
            for aerodrome_id in 0..1000 {
                actions.push(AiAction::CreateBase { aerodrome_id });
            }
        }
        if self.cash >= 150_000
            && self.bases.len() > 0
            && self.planes.len() > self.landing_rights.len()
        {
            for aerodrome_id in 0..1000 {
                actions.push(AiAction::BuyLandingRights { aerodrome_id });
            }
        }
        if self.cash >= 500 {
            for plane_id in &self.planes {
                for origin_id in &self.bases {
                    for destination_id in &self.landing_rights {
                        if origin_id != destination_id {
                            actions.push(AiAction::ScheduleFlight {
                                plane_id: *plane_id,
                                origin_id: *origin_id,
                                destination_id: *destination_id,
                            });
                        }
                    }
                }
            }
        }
        actions
    }
}

struct AiAgent<'a> {
    state: AiState,
    simulation: &'a mut Simulation,
    plane_types: Vec<PlaneType>,
    aerodromes: Vec<Aerodrome>,
}

impl<'a> AiAgent<'a> {
    pub fn new(
        simulation: &'a mut Simulation,
        plane_types: Vec<PlaneType>,
        aerodromes: Vec<Aerodrome>,
    ) -> Self {
        let environment = &simulation.environment;
        Self {
            state: AiState {
                cash: environment.company_finances.cash(environment.timestamp) as u64,
                planes: environment.planes.iter().map(|plane| plane.id).collect(),
                bases: environment.bases.iter().map(|base| base.id).collect(),
                landing_rights: environment
                    .landing_rights
                    .iter()
                    .map(|landing_rights| landing_rights.aerodrome.id)
                    .collect(),
            },
            simulation,
            plane_types,
            aerodromes,
        }
    }

    fn update_state(&mut self) {
        let environment = &self.simulation.environment;
        self.state = AiState {
            cash: environment.company_finances.cash(environment.timestamp) as u64,
            planes: environment.planes.iter().map(|plane| plane.id).collect(),
            bases: environment.bases.iter().map(|base| base.id).collect(),
            landing_rights: environment
                .landing_rights
                .iter()
                .map(|landing_rights| landing_rights.aerodrome.id)
                .collect(),
        }
    }
}

impl Agent<AiState> for AiAgent<'_> {
    fn current_state(&self) -> &AiState {
        &self.state
    }

    fn take_action(&mut self, action: &<AiState as State>::A) {
        let command = action.to_command(
            &self.simulation.environment,
            &self.plane_types,
            &self.aerodromes,
        );
        println!("Taking action: {:?}", action);
        match command {
            Some(command) => self.simulation.add_command(command),
            None => {}
        }
        self.simulation.update(Duration::from_secs(1));
        self.update_state();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use rurel::{
        strategy::{explore::RandomExploration, learn::QLearning, terminate::FixedIterations},
        AgentTrainer,
    };

    use crate::{
        config::{load_airports, PlanesConfig},
        model::{
            commands::{BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand},
            Aerodrome,
        },
        simulation::Simulation,
    };

    #[test]
    fn test_ai_agent() {
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

        let mut agent = super::AiAgent::new(&mut simulation, planes_config.planes, aerodromes);

        let mut trainer = AgentTrainer::new();
        trainer.train(
            &mut agent,
            &QLearning::new(0.2, 0.01, 2.0),
            &mut FixedIterations::new(1000),
            &RandomExploration::new(),
        );

        println!("{:?}", agent.state);
        println!("Planes: {:#?}", simulation.environment.planes);
        println!("Bases: {:#?}", simulation.environment.bases);

    }
}
