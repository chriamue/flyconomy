use crate::model::{
    commands::{
        BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand,
        ScheduleFlightCommand, SellLandingRightsCommand, SellPlaneCommand,
    },
    Aerodrome, Environment, PlaneType,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AiAction {
    NoOp,
    BuyPlane {
        plane_id: u64,
        plane_type: u32,
        base_id: u64,
    },
    CreateBase {
        base_id: u64,
        aerodrome_id: u64,
    },
    BuyLandingRights {
        landing_rights_id: u64,
        aerodrome_id: u64,
    },
    ScheduleFlight {
        plane_id: u64,
        origin_id: u64,
        destination_id: u64,
    },
    SellLandingRights {
        landing_rights_id: u64,
    },
    SellPlane {
        plane_id: u64,
    },
}

impl Into<[f32; 10]> for AiAction {
    fn into(self) -> [f32; 10] {
        match self {
            AiAction::NoOp => [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            AiAction::BuyPlane {
                plane_id,
                plane_type,
                base_id,
            } => [
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                plane_id as f32,
                plane_type as f32,
                base_id as f32,
            ],
            AiAction::CreateBase {
                base_id,
                aerodrome_id,
            } => [
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                base_id as f32,
                aerodrome_id as f32,
                0.0,
            ],
            AiAction::BuyLandingRights {
                landing_rights_id,
                aerodrome_id,
            } => [
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                landing_rights_id as f32,
                aerodrome_id as f32,
                0.0,
            ],
            AiAction::ScheduleFlight {
                plane_id,
                origin_id,
                destination_id,
            } => [
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                plane_id as f32,
                origin_id as f32,
                destination_id as f32,
            ],
            AiAction::SellLandingRights { landing_rights_id } => [
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                landing_rights_id as f32,
                0.0,
                0.0,
            ],
            AiAction::SellPlane { plane_id } => {
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, plane_id as f32, 0.0, 0.0]
            }
        }
    }
}

impl From<[f32; 10]> for AiAction {
    fn from(v: [f32; 10]) -> Self {
        // Find the index of the maximum value
        let max_index = v[0..4]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        match max_index {
            0 => AiAction::NoOp,
            1 => AiAction::BuyPlane {
                plane_id: v[7] as u64,
                plane_type: v[8] as u32,
                base_id: v[9] as u64,
            },
            2 => AiAction::CreateBase {
                base_id: v[7] as u64,
                aerodrome_id: v[8] as u64,
            },
            3 => AiAction::BuyLandingRights {
                landing_rights_id: v[7] as u64,
                aerodrome_id: v[8] as u64,
            },
            4 => AiAction::ScheduleFlight {
                plane_id: v[7] as u64,
                origin_id: v[8] as u64,
                destination_id: v[9] as u64,
            },
            5 => AiAction::SellLandingRights {
                landing_rights_id: v[7] as u64,
            },
            6 => AiAction::SellPlane {
                plane_id: v[7] as u64,
            },
            _ => panic!("Invalid action index"),
        }
    }
}

impl AiAction {
    pub fn to_command(
        &self,
        environment: &Environment,
        aerodromes: &Vec<Aerodrome>,
        plane_types: &Vec<PlaneType>,
    ) -> Option<Box<dyn Command>> {
        match self {
            AiAction::BuyPlane {
                plane_id,
                plane_type,
                base_id,
            } => Some(Box::new(BuyPlaneCommand {
                plane_id: *plane_id,
                plane_type: plane_types[*plane_type as usize].clone(),
                home_base_id: *base_id,
            })),
            AiAction::CreateBase {
                base_id,
                aerodrome_id,
            } => {
                match aerodromes
                    .iter()
                    .find(|aerodrome| aerodrome.id == *aerodrome_id)
                {
                    Some(aerodrome) => Some(Box::new(CreateBaseCommand {
                        base_id: *base_id,
                        aerodrome: aerodrome.clone(),
                    })),
                    None => return None,
                }
            }
            AiAction::BuyLandingRights {
                landing_rights_id,
                aerodrome_id,
            } => {
                match aerodromes
                    .iter()
                    .find(|aerodrome| aerodrome.id == *aerodrome_id)
                {
                    Some(aerodrome) => Some(Box::new(BuyLandingRightsCommand {
                        landing_rights_id: *landing_rights_id,
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
                            flight_id: ScheduleFlightCommand::generate_id(),
                            airplane,
                            origin_aerodrome,
                            stopovers: vec![destination_aerodrome],
                            departure_time: environment.timestamp,
                        }))
                    }
                    _ => None, // return None if any of the required components are not found
                }
            }
            AiAction::SellLandingRights { landing_rights_id } => {
                match environment
                    .landing_rights
                    .iter()
                    .find(|landing_rights| landing_rights.id == *landing_rights_id)
                {
                    Some(landing_rights) => Some(Box::new(SellLandingRightsCommand {
                        landing_rights_id: landing_rights.id,
                    })),
                    None => return None,
                }
            }
            AiAction::SellPlane { plane_id } => {
                match environment
                    .planes
                    .iter()
                    .find(|plane| plane.id == *plane_id)
                {
                    Some(plane) => Some(Box::new(SellPlaneCommand { plane_id: plane.id })),
                    None => return None,
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
                plane_id: command.plane_id,
                plane_type: command.plane_type.id,
                base_id: command.home_base_id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<CreateBaseCommand>() {
            AiAction::CreateBase {
                base_id: command.base_id,
                aerodrome_id: command.aerodrome.id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<BuyLandingRightsCommand>() {
            AiAction::BuyLandingRights {
                landing_rights_id: command.landing_rights_id,
                aerodrome_id: command.aerodrome.id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<ScheduleFlightCommand>() {
            AiAction::ScheduleFlight {
                plane_id: command.airplane.id,
                origin_id: command.origin_aerodrome.id,
                destination_id: command.stopovers[0].id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<SellLandingRightsCommand>() {
            AiAction::SellLandingRights {
                landing_rights_id: command.landing_rights_id,
            }
        } else if let Some(command) = value.as_any().downcast_ref::<SellPlaneCommand>() {
            AiAction::SellPlane {
                plane_id: command.plane_id,
            }
        } else {
            AiAction::NoOp
        }
    }
}
