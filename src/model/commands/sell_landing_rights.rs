use std::any::Any;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::Environment;

use super::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellLandingRightsCommand {
    pub landing_rights_id: u32,
}

#[derive(Debug, Error)]
pub enum SellLandingRightsError {
    #[error("Landing rights does not exist")]
    NotExist,
}

impl Command for SellLandingRightsCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let landing_rights = environment
            .landing_rights
            .iter()
            .find(|lr| lr.id == self.landing_rights_id);
        if landing_rights.is_none() {
            return Err(Box::new(SellLandingRightsError::NotExist));
        }

        environment
            .landing_rights
            .retain(|lr| lr.id != self.landing_rights_id);

        environment.company_finances.add_income(
            environment.timestamp,
            environment.config.landing_rights_cost,
        );
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
    fn test_sell_landing_rights_not_exist() {
        let mut environment = Environment::default();

        let cmd = SellLandingRightsCommand {
            landing_rights_id: 42,
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<SellLandingRightsError>().unwrap();
                assert!(matches!(*err, SellLandingRightsError::NotExist));
            }
            _ => panic!("Expected an error"),
        }
    }
}
