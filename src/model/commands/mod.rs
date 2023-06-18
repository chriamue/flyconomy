use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};
use thiserror::Error;

use super::{flight::FlightState, Aerodrome, AirPlane, Environment, PlaneType};
use crate::model::{Base, Flight, LandingRights};

pub trait Command: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>>;
}

static PLANE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct BuyPlaneCommand {
    pub plane_type: PlaneType,
    pub home_base_id: u64,
}

#[derive(Debug, Error)]
pub enum BuyPlaneError {
    #[error("Insufficient funds: needed {needed}, but have {has}")]
    InsufficientFunds { needed: f64, has: f64 },
    #[error("Base not found")]
    BaseNotFound { base_id: u64 },
}

impl Command for BuyPlaneCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if environment.company_finances.cash < self.plane_type.cost as f64 {
            return Err(Box::new(BuyPlaneError::InsufficientFunds {
                needed: self.plane_type.cost as f64,
                has: environment.company_finances.cash,
            }));
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

        match environment
            .bases
            .iter_mut()
            .find(|base| base.id == self.home_base_id)
        {
            Some(base) => base.airplane_ids.push(airplane_id),
            None => {
                return Err(Box::new(BuyPlaneError::BaseNotFound {
                    base_id: self.home_base_id,
                }))
            }
        }

        environment.company_finances.cash -= self.plane_type.cost as f64;
        environment.planes.push(airplane);
        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

static BASE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct CreateBaseCommand {
    pub aerodrome: Aerodrome,
}

#[derive(Debug, Error)]
pub enum CreateBaseError {
    #[error("Insufficient funds to create base: needed {needed}, but have {has}")]
    InsufficientFunds { needed: f64, has: f64 },
}

impl Command for CreateBaseCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if environment.company_finances.cash < environment.config.base_cost {
            return Err(Box::new(CreateBaseError::InsufficientFunds {
                needed: environment.config.base_cost,
                has: environment.company_finances.cash,
            }));
        }

        environment.company_finances.cash -= environment.config.base_cost;
        let base_id = BASE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        environment.bases.push(Base {
            id: base_id.try_into().unwrap(),
            aerodrome: self.aerodrome.clone(),
            airplane_ids: vec![],
        });
        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

static LANDING_RIGHTS_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct BuyLandingRightsCommand {
    pub aerodrome: Aerodrome,
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
        if environment.company_finances.cash < environment.config.landing_rights_cost {
            return Err(Box::new(BuyLandingRightsError::InsufficientFunds {
                needed: environment.config.landing_rights_cost,
                has: environment.company_finances.cash,
            }));
        }
        environment.company_finances.cash -= environment.config.landing_rights_cost;
        environment.landing_rights.push(LandingRights {
            aerodrome: self.aerodrome.clone(),
            id: LANDING_RIGHTS_ID_COUNTER
                .fetch_add(1, Ordering::SeqCst)
                .try_into()
                .unwrap(),
        });
        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

static FLIGHT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct ScheduleFlightCommand {
    pub airplane: AirPlane,
    pub origin_aerodrome: Aerodrome,
    pub destination_aerodrome: Aerodrome,
    pub departure_time: u64,
}

#[derive(Debug, Error)]
pub enum ScheduleFlightError {
    #[error("Cannot schedule the flight because the airplane is already in use")]
    AirplaneInUse,
    #[error("Cannot schedule the flight because the distance is beyond the airplane's range")]
    DistanceBeyondRange,
}

impl Command for ScheduleFlightCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // Check if the airplane is already in use for an ongoing flight
        let airplane_id = self.airplane.id;
        let is_airplane_in_use = environment
            .flights
            .iter()
            .any(|flight| flight.airplane.id == airplane_id && flight.state != FlightState::Landed);

        // If the airplane is in use, return an error
        if is_airplane_in_use {
            return Err(Box::new(ScheduleFlightError::AirplaneInUse));
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

        // Check if the distance is within the airplane's range
        let distance = flight.calculate_distance();
        if distance > self.airplane.plane_type.range as f64 {
            return Err(Box::new(ScheduleFlightError::DistanceBeyondRange));
        }

        let profit = flight.calculate_profit();

        environment.flights.push(flight);
        environment.company_finances.total_income += profit;
        environment.company_finances.cash += profit as f64;

        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
