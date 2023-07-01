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
