use super::{Environment, PlaneType};

pub trait Command: Send + Sync {
    fn execute(&self, environment: &mut Environment) -> Option<String>;
}

pub struct BuyPlaneCommand {
    pub plane_type: PlaneType,
}

impl Command for BuyPlaneCommand {
    fn execute(&self, environment: &mut Environment) -> Option<String> {
        if environment.company_finances.cash < self.plane_type.cost as f64 {
            return Some(format!(
                "Not enough cash to buy plane: {}",
                self.plane_type.name
            ));
        }
        environment.company_finances.cash -= self.plane_type.cost as f64;
        environment.planes.push(self.plane_type.clone());
        None
    }
}
