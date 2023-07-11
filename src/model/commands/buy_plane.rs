use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{AirPlane, Environment, PlaneType};

use super::Command;

static PLANE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyPlaneCommand {
    pub plane_id: u64,
    pub plane_type: PlaneType,
    pub home_base_id: u64,
}

impl BuyPlaneCommand {
    pub fn generate_id() -> u64 {
        PLANE_ID_COUNTER
            .fetch_add(1, Ordering::SeqCst)
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, Error)]
pub enum BuyPlaneError {
    #[error("Insufficient funds: needed {needed}, but have {has}")]
    InsufficientFunds { needed: f64, has: f64 },
    #[error("Base not found")]
    BaseNotFound { base_id: u64 },
    #[error("No space at base: {name}")]
    NoSpaceAtBase { name: String },
}

impl Command for BuyPlaneCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if environment.company_finances.cash(environment.timestamp) < self.plane_type.cost as f64 {
            return Err(Box::new(BuyPlaneError::InsufficientFunds {
                needed: self.plane_type.cost as f64,
                has: environment.company_finances.cash(environment.timestamp),
            }));
        }
        let airplane = AirPlane {
            id: self.plane_id,
            base_id: self.home_base_id,
            plane_type: self.plane_type.clone(),
        };

        match environment
            .bases
            .iter_mut()
            .find(|base| base.id == self.home_base_id)
        {
            Some(base) => {
                if base.airplane_ids.len() >= 5 {
                    return Err(Box::new(BuyPlaneError::NoSpaceAtBase {
                        name: base.aerodrome.name.clone(),
                    }));
                }
                base.airplane_ids.push(airplane.id);
            }
            None => {
                return Err(Box::new(BuyPlaneError::BaseNotFound {
                    base_id: self.home_base_id,
                }))
            }
        }
        environment.planes.push(airplane);

        environment
            .company_finances
            .add_expense(environment.timestamp, self.plane_type.cost.into());
        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::model::PlaneType;

    use super::*;

    #[test]
    fn test_buy_plane_insufficient_funds() {
        let mut environment = Environment::default();

        environment.company_finances.income.clear();
        environment.company_finances.add_income(0, 100.0);

        let plane_type = PlaneType::default();

        let cmd = BuyPlaneCommand {
            plane_id: BuyPlaneCommand::generate_id(),
            plane_type: plane_type.clone(),
            home_base_id: 0,
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<BuyPlaneError>().unwrap();
                assert!(matches!(*err, BuyPlaneError::InsufficientFunds { .. }));
            }
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_buy_plane_base_not_found() {
        let mut environment = Environment::default();

        let plane_type = PlaneType::default();

        let cmd = BuyPlaneCommand {
            plane_id: BuyPlaneCommand::generate_id(),
            plane_type: plane_type.clone(),
            home_base_id: 0, // Base with id 0 does not exist
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<BuyPlaneError>().unwrap();
                assert!(matches!(*err, BuyPlaneError::BaseNotFound { .. }));
            }
            _ => panic!("Expected an error"),
        }
    }
}
