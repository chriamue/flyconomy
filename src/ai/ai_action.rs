use crate::model::{
    commands::{
        BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand, ScheduleFlightCommand,
    },
    Aerodrome, Environment, PlaneType,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AiAction {
    NoOp,
    BuyPlane {
        plane_type: u32,
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
                match (
                    environment
                        .planes
                        .iter()
                        .find(|plane| plane.id == *plane_id),
                    environment.bases.iter().find(|base| base.id == *origin_id),
                    environment
                        .landing_rights
                        .iter()
                        .find(|landing_rights| landing_rights.aerodrome.id == *destination_id),
                ) {
                    (Some(airplane), Some(base), Some(landing_rights)) => {
                        let airplane = airplane.clone();
                        let origin_aerodrome = base.aerodrome.clone();
                        let destination_aerodrome = landing_rights.aerodrome.clone();
                        Some(Box::new(ScheduleFlightCommand {
                            airplane,
                            origin_aerodrome,
                            destination_aerodrome,
                            departure_time: environment.timestamp,
                        }))
                    }
                    _ => None, // return None if any of the required components are not found
                }
            }

            AiAction::NoOp => None,
        }
    }
}

impl From<&Box<dyn Command>> for AiAction {
    fn from(value: &Box<dyn Command>) -> Self {
        if let Some(command) = value.as_any().downcast_ref::<BuyPlaneCommand>() {
            AiAction::BuyPlane {
                plane_type: command.plane_type.id,
                base_id: command.home_base_id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<CreateBaseCommand>() {
            AiAction::CreateBase {
                aerodrome_id: command.aerodrome.id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<BuyLandingRightsCommand>() {
            AiAction::BuyLandingRights {
                aerodrome_id: command.aerodrome.id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<ScheduleFlightCommand>() {
            AiAction::ScheduleFlight {
                plane_id: command.airplane.id,
                origin_id: command.origin_aerodrome.id,
                destination_id: command.destination_aerodrome.id,
            }
        } else {
            AiAction::NoOp
        }
    }
}
