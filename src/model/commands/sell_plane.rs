use super::Command;
use crate::model::Environment;
use serde::{Deserialize, Serialize};
use std::any::Any;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellPlaneCommand {
    pub plane_id: u64,
}

#[derive(Debug, Error)]
pub enum SellPlaneError {
    #[error("Landing rights does not exist")]
    NotExist,
}

impl Command for SellPlaneCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let airplane = environment.planes.iter().find(|lr| lr.id == self.plane_id);
        if airplane.is_none() {
            return Err(Box::new(SellPlaneError::NotExist));
        }

        environment.planes.retain(|lr| lr.id != self.plane_id);

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

        let cmd = SellPlaneCommand { plane_id: 42 };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<SellPlaneError>().unwrap();
                assert!(matches!(*err, SellPlaneError::NotExist));
            }
            _ => panic!("Expected an error"),
        }
    }
}
