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
    ) -> Box<dyn Command> {
        match self {
            AiAction::BuyPlane {
                plane_type,
                base_id,
            } => Box::new(BuyPlaneCommand {
                plane_type: plane_types[*plane_type as usize].clone(),
                home_base_id: *base_id,
            }),
            AiAction::CreateBase { aerodrome_id } => {
                let aerodrome = aerodromes
                    .iter()
                    .find(|aerodrome| aerodrome.id == *aerodrome_id)
                    .unwrap()
                    .clone();
                Box::new(CreateBaseCommand { aerodrome })
            }
            AiAction::BuyLandingRights { aerodrome_id } => {
                let aerodrome = aerodromes
                    .iter()
                    .find(|aerodrome| aerodrome.id == *aerodrome_id)
                    .unwrap()
                    .clone();
                Box::new(BuyLandingRightsCommand { aerodrome })
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
                    .bases
                    .iter()
                    .find(|base| base.id == *destination_id)
                    .unwrap()
                    .aerodrome
                    .clone();

                Box::new(ScheduleFlightCommand {
                    airplane,
                    origin_aerodrome,
                    destination_aerodrome,
                    departure_time: environment.timestamp,
                })
            }
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
        let mut actions = vec![];
        if self.cash >= 100_000 {
            for base_id in &self.bases {
                for plane_type in 0..3 {
                    actions.push(AiAction::BuyPlane {
                        plane_type,
                        base_id: *base_id,
                    });
                }
            }
        }
        if self.cash >= 400_000 {
            for aerodrome_id in &self.landing_rights {
                actions.push(AiAction::CreateBase {
                    aerodrome_id: *aerodrome_id,
                });
            }
        }
        if self.cash >= 100_000 {
            for aerodrome_id in &self.landing_rights {
                actions.push(AiAction::BuyLandingRights {
                    aerodrome_id: *aerodrome_id,
                });
            }
        }
        if self.cash >= 500 {
            for plane_id in &self.planes {
                for origin_id in &self.bases {
                    for destination_id in &self.bases {
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
        self.simulation.add_command(command);
        self.update_state();
    }
}
