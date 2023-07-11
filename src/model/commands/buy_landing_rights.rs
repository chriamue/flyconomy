use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{Aerodrome, Environment, LandingRights};

use super::Command;

static LANDING_RIGHTS_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyLandingRightsCommand {
    pub landing_rights_id: u64,
    pub aerodrome: Aerodrome,
}

impl BuyLandingRightsCommand {
    pub fn generate_id() -> u64 {
        LANDING_RIGHTS_ID_COUNTER
            .fetch_add(1, Ordering::SeqCst)
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, Error)]
pub enum BuyLandingRightsError {
    #[error("Insufficient funds to buy landing rights: needed {needed}, but have {has}")]
    InsufficientFunds { needed: f64, has: f64 },
}

impl Command for BuyLandingRightsCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if environment.company_finances.cash(environment.timestamp)
            < environment.config.landing_rights_cost
        {
            return Err(Box::new(BuyLandingRightsError::InsufficientFunds {
                needed: environment.config.landing_rights_cost,
                has: environment.company_finances.cash(environment.timestamp),
            }));
        }
        environment.company_finances.add_expense(
            environment.timestamp,
            environment.config.landing_rights_cost,
        );
        environment.landing_rights.push(LandingRights {
            aerodrome: self.aerodrome.clone(),
            id: self.landing_rights_id.try_into()?,
        });
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
    use super::*;

    #[test]
    fn test_buy_landing_rights_insufficient_funds() {
        let mut environment = Environment::default();
        environment.company_finances.income.clear();
        environment.company_finances.add_income(0, 200.0);

        let aerodrome = Aerodrome::default();

        let cmd = BuyLandingRightsCommand {
            landing_rights_id: BuyLandingRightsCommand::generate_id(),
            aerodrome: aerodrome.clone(),
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<BuyLandingRightsError>().unwrap();
                assert!(matches!(
                    *err,
                    BuyLandingRightsError::InsufficientFunds { .. }
                ));
            }
            _ => panic!("Expected an error"),
        }
    }
}
