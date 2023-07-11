use rurel::mdp::State;

use crate::model::{
    commands::{BuyLandingRightsCommand, BuyPlaneCommand, CreateBaseCommand},
    Timestamp,
};

use super::AiAction;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AiState {
    pub cash: u64,
    pub total_turnover: u64,
    pub planes: Vec<u64>,
    pub bases: Vec<u64>,
    pub landing_rights: Vec<u64>,
    pub timestamp: Timestamp,
    pub error_indicator: u64,
}

impl Into<[f32; 7]> for AiState {
    fn into(self) -> [f32; 7] {
        [
            self.cash as f32,
            self.total_turnover as f32,
            self.planes.len() as f32,
            self.bases.len() as f32,
            self.landing_rights.len() as f32,
            (self.timestamp as f32).log2(),
            self.error_indicator as f32,
        ]
    }
}

// ai state from environment
impl From<&crate::model::Environment> for AiState {
    fn from(environment: &crate::model::Environment) -> Self {
        let cash = environment.company_finances.cash(environment.timestamp) as u64;
        let total_turnover = (environment
            .company_finances
            .total_income(environment.timestamp)
            + environment
                .company_finances
                .total_expenses(environment.timestamp)) as u64;
        Self {
            cash,
            total_turnover,
            planes: environment.planes.iter().map(|plane| plane.id).collect(),
            bases: environment.bases.iter().map(|base| base.id).collect(),
            landing_rights: environment
                .landing_rights
                .iter()
                .map(|landing_rights| landing_rights.aerodrome.id)
                .collect(),
            timestamp: environment.timestamp,
            error_indicator: environment.get_errors_indicator(),
        }
    }
}

impl State for AiState {
    type A = AiAction;

    fn reward(&self) -> f64 {
        let mut reward = 0.0;
        reward += self.cash as f64 / 1_000_000.0;
        reward += self.total_turnover as f64 / 1_000_000.0;
        reward += (self.planes.len() * 5) as f64;
        reward += self.bases.len() as f64;
        reward += (self.landing_rights.len() * 2) as f64;
        reward -= self.error_indicator as f64;
        reward
    }

    fn actions(&self) -> Vec<Self::A> {
        let mut actions = vec![AiAction::NoOp];
        if self.cash >= 350_000 {
            let plane_id = BuyPlaneCommand::generate_id();
            for base_id in &self.bases {
                for plane_type in 0..3 {
                    actions.push(AiAction::BuyPlane {
                        plane_id,
                        plane_type,
                        base_id: *base_id,
                    });
                }
            }
        }
        if self.cash >= 800_000 {
            let base_id = CreateBaseCommand::generate_id();
            for aerodrome_id in 0..8000 {
                actions.push(AiAction::CreateBase {
                    base_id,
                    aerodrome_id,
                });
            }
        }
        if self.cash >= 150_000
            && self.bases.len() > 0
            && self.planes.len() > self.landing_rights.len()
        {
            let landing_rights_id = BuyLandingRightsCommand::generate_id();
            for aerodrome_id in 0..8000 {
                actions.push(AiAction::BuyLandingRights {
                    landing_rights_id,
                    aerodrome_id,
                });
            }
        }
        if self.cash >= 500 {
            for plane_id in &self.planes {
                for origin_id in &self.bases {
                    // destination_id is either a base or a landing rights
                    for destination_id in self.bases.iter().chain(&self.landing_rights) {
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
        if self.cash < 200_000 {
            for landing_rights_id in &self.landing_rights {
                actions.push(AiAction::SellLandingRights {
                    landing_rights_id: *landing_rights_id as u32,
                });
            }
        }
        actions
    }
}
