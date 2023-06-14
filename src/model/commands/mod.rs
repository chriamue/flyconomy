use std::sync::atomic::{AtomicUsize, Ordering};

use crate::model::{Base, Flight, LandingRights};

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

static LANDING_RIGHTS_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct BuyLandingRightsCommand {
    pub aerodrome: Aerodrome,
}

impl Command for BuyLandingRightsCommand {
    fn execute(&self, environment: &mut Environment) -> Option<String> {
        const LANDING_RIGHTS_COST: f64 = 100_000.0;
        if environment.company_finances.cash < LANDING_RIGHTS_COST {
            return Some(format!("Not enough cash to buy landing rights"));
        }
        environment.company_finances.cash -= LANDING_RIGHTS_COST;
        environment.landing_rights.push(LandingRights {
            aerodrome: self.aerodrome.clone(),
            id: LANDING_RIGHTS_ID_COUNTER
                .fetch_add(1, Ordering::SeqCst)
                .try_into()
                .unwrap(),
        });
        None
    }
}

static FLIGHT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct ScheduleFlightCommand {
    pub airplane: AirPlane,
    pub origin_aerodrome: Aerodrome,
    pub destination_aerodrome: Aerodrome,
}

impl Command for ScheduleFlightCommand {
    fn execute(&self, environment: &mut Environment) -> Option<String> {
        let flight_id = FLIGHT_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let flight = Flight {
            flight_id: flight_id.try_into().unwrap(),
            airplane: self.airplane.clone(),
            origin_aerodrome: self.origin_aerodrome.clone(),
            destination_aerodrome: self.destination_aerodrome.clone(),
        };

        let profit = flight.calculate_profit();

        environment.flights.push(flight);
        environment.company_finances.total_income += profit;
        environment.company_finances.cash += profit as f64;

        None
    }
}
