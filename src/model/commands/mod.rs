use std::sync::atomic::{AtomicUsize, Ordering};

use crate::model::{Base, Flight, LandingRights};

use super::{flight::FlightState, Aerodrome, AirPlane, Environment, PlaneType};

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
            .find(|base| base.id == self.home_base_id)?
            .airplane_ids
            .push(airplane_id);

        environment.company_finances.cash -= self.plane_type.cost as f64;
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
        if environment.company_finances.cash < environment.config.base_cost {
            return Some(format!("Not enough cash to create base"));
        }
        environment.company_finances.cash -= environment.config.base_cost;
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
        if environment.company_finances.cash < environment.config.landing_rights_cost {
            return Some(format!("Not enough cash to buy landing rights"));
        }
        environment.company_finances.cash -= environment.config.landing_rights_cost;
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
    pub departure_time: u64,
}

impl Command for ScheduleFlightCommand {
    fn execute(&self, environment: &mut Environment) -> Option<String> {
        // Check if the airplane is already in use for an ongoing flight
        let airplane_id = self.airplane.id;
        let is_airplane_in_use = environment
            .flights
            .iter()
            .any(|flight| flight.airplane.id == airplane_id && flight.state != FlightState::Landed);

        // If the airplane is in use, return an error message
        if is_airplane_in_use {
            return Some(
                "Cannot schedule the flight because the airplane is already in use".to_string(),
            );
        }
        let flight_id = FLIGHT_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let flight = Flight {
            flight_id: flight_id.try_into().unwrap(),
            airplane: self.airplane.clone(),
            origin_aerodrome: self.origin_aerodrome.clone(),
            destination_aerodrome: self.destination_aerodrome.clone(),
            departure_time: self.departure_time,
            arrival_time: None,
            state: FlightState::Scheduled,
        };

        let profit = flight.calculate_profit();

        environment.flights.push(flight);
        environment.company_finances.total_income += profit;
        environment.company_finances.cash += profit as f64;

        None
    }
}
