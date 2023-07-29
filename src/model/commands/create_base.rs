use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{Aerodrome, Base, Environment};

use super::Command;

static BASE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBaseCommand {
    pub base_id: u64,
    pub aerodrome: Aerodrome,
}

impl CreateBaseCommand {
    pub fn generate_id() -> u64 {
        BASE_ID_COUNTER
            .fetch_add(1, Ordering::SeqCst)
            .try_into()
            .unwrap()
    }

    pub fn base_cost(&self, environment: &Environment) -> f64 {
        match self.aerodrome.passengers {
            Some(passengers) => environment.config.base_cost + passengers as f64 / 20.0,
            None => environment.config.base_cost,
        }
    }
}

#[derive(Debug, Error)]
pub enum CreateBaseError {
    #[error("Insufficient funds to create base: needed {needed}, but have {has}")]
    InsufficientFunds { needed: f64, has: f64 },
    #[error("Base already exists for the given aerodrome: {0}")]
    BaseAlreadyExists(String),
}

impl Command for CreateBaseCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if environment
            .bases
            .iter()
            .any(|base| base.aerodrome.code == self.aerodrome.code)
        {
            return Err(Box::new(CreateBaseError::BaseAlreadyExists(
                self.aerodrome.name.clone(),
            )));
        }
        if environment.company_finances.cash(environment.timestamp) < self.base_cost(environment) {
            return Err(Box::new(CreateBaseError::InsufficientFunds {
                needed: self.base_cost(environment),
                has: environment.company_finances.cash(environment.timestamp),
            }));
        }

        environment
            .company_finances
            .add_expense(environment.timestamp, self.base_cost(environment));
        environment.bases.push(Base {
            id: self.base_id,
            aerodrome: self.aerodrome.clone(),
            airplane_ids: vec![],
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
    fn test_create_base_insufficient_funds() {
        let mut environment = Environment::default();
        environment.company_finances.income.clear();
        environment.company_finances.add_income(0, 500.0); // Not enough for a base

        let aerodrome = Aerodrome::default();

        let cmd = CreateBaseCommand {
            base_id: CreateBaseCommand::generate_id(),
            aerodrome: aerodrome.clone(),
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<CreateBaseError>().unwrap();
                assert!(matches!(*err, CreateBaseError::InsufficientFunds { .. }));
            }
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_create_base_base_already_exists() {
        let mut environment = Environment::default();
        environment.company_finances.income.clear();
        environment.company_finances.add_income(0, 100000.0);

        let aerodrome = Aerodrome::default();
        environment.bases.push(Base {
            id: 1,
            aerodrome: aerodrome.clone(),
            airplane_ids: vec![],
        });

        let cmd = CreateBaseCommand {
            base_id: CreateBaseCommand::generate_id(),
            aerodrome: aerodrome.clone(),
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<CreateBaseError>().unwrap();
                assert!(matches!(*err, CreateBaseError::BaseAlreadyExists(..)));
            }
            _ => panic!("Expected an error"),
        }
    }
}
