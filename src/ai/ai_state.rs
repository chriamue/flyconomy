use rurel::mdp::State;

use crate::model::Timestamp;

use super::AiAction;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AiState {
    pub cash: u64,
    pub total_turnover: u64,
    pub planes: Vec<u64>,
    pub bases: Vec<u64>,
    pub landing_rights: Vec<u64>,
    pub timestamp: Timestamp,
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
        reward
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
            for aerodrome_id in 0..8000 {
                actions.push(AiAction::CreateBase { aerodrome_id });
            }
        }
        if self.cash >= 150_000
            && self.bases.len() > 0
            && self.planes.len() > self.landing_rights.len()
        {
            for aerodrome_id in 0..8000 {
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
