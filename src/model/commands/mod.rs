use std::sync::atomic::{AtomicUsize, Ordering};

use crate::model::Base;

use super::{Aerodrome, AirPlane, Environment, PlaneType};

pub trait Command: Send + Sync {
    fn execute(&self, environment: &mut Environment) -> Option<String>;
}

static PLANE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct BuyPlaneCommand {
    pub plane_type: PlaneType,
    pub home_base_id: u64,
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
        let airplane_id: u64 = PLANE_ID_COUNTER
            .fetch_add(1, Ordering::SeqCst)
            .try_into()
            .unwrap();
        let airplane = AirPlane {
            id: airplane_id,
            base_id: self.home_base_id,
            plane_type: self.plane_type.clone(),
        };
        environment
            .bases
            .iter_mut()
            .find(|base| base.id == self.home_base_id)
            .unwrap()
            .airplane_ids
            .push(airplane_id);

        environment.planes.push(airplane);
        None
    }
}

static BASE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct CreateBaseCommand {
    pub aerodrome: Aerodrome,
}

impl Command for CreateBaseCommand {
    fn execute(&self, environment: &mut Environment) -> Option<String> {
        const BASE_COST: f64 = 400_000.0;
        if environment.company_finances.cash < BASE_COST {
            return Some(format!("Not enough cash to create base"));
        }
        environment.company_finances.cash -= BASE_COST;
        let base_id = BASE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        environment.bases.push(Base {
            id: base_id.try_into().unwrap(),
            aerodrome: self.aerodrome.clone(),
            airplane_ids: vec![],
        });
        None
    }
}
