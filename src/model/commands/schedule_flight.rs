use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{Aerodrome, AirPlane, Environment, Flight, FlightState, Timestamp};

use super::Command;

static FLIGHT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleFlightCommand {
    pub flight_id: u64,
    pub airplane: AirPlane,
    pub origin_aerodrome: Aerodrome,
    pub stopovers: Vec<Aerodrome>,
    pub departure_time: Timestamp,
    pub interest_score: f64,
}

impl ScheduleFlightCommand {
    pub fn generate_id() -> u64 {
        FLIGHT_ID_COUNTER
            .fetch_add(1, Ordering::SeqCst)
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, Error)]
pub enum ScheduleFlightError {
    #[error("Cannot schedule the flight because the airplane is already in use")]
    AirplaneInUse,
    #[error("Cannot schedule the flight because the distance is beyond the airplane's range")]
    DistanceBeyondRange,
    #[error("The airplane is not located at the origin aerodrome")]
    AirplaneNotLocatedAtOrigin,
}

impl Command for ScheduleFlightCommand {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let airplane_id = self.airplane.id;

        let is_airplane_in_use = environment.flights.iter().any(|flight| {
            flight.airplane.id == airplane_id && flight.state != FlightState::Finished
        });

        // If the airplane is in use, return an error
        if is_airplane_in_use {
            return Err(Box::new(ScheduleFlightError::AirplaneInUse));
        }

        let flight = Flight {
            flight_id: self.flight_id,
            airplane: self.airplane.clone(),
            origin_aerodrome: self.origin_aerodrome.clone(),
            stopovers: self.stopovers.clone(),
            departure_time: self.departure_time,
            segment_departure_time: self.departure_time,
            arrival_time: None,
            state: FlightState::Scheduled,
            interest_score: self.interest_score,
        };

        // Check if the distance is within the airplane's range
        if !flight.is_plane_range_sufficient() {
            return Err(Box::new(ScheduleFlightError::DistanceBeyondRange));
        }

        let is_airplane_located_at_origin = environment.bases.iter().any(|base| {
            self.origin_aerodrome.id == base.aerodrome.id
                && base.airplane_ids.contains(&airplane_id)
        });
        if !is_airplane_located_at_origin {
            return Err(Box::new(ScheduleFlightError::AirplaneNotLocatedAtOrigin));
        }

        let profit = flight.calculate_profit();

        environment.flights.push(flight);
        environment
            .company_finances
            .add_income(environment.timestamp, profit);

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
    use crate::model::{AirPlane, Flight, FlightState};

    use super::*;

    #[test]
    fn test_schedule_flight_airplane_in_use() {
        let mut environment = Environment::default();

        let airplane = AirPlane::default();
        let aerodrome = Aerodrome::default();

        // Add a flight to the environment using the same airplane
        environment.flights.push(Flight {
            flight_id: 1,
            airplane: airplane.clone(),
            origin_aerodrome: aerodrome.clone(),
            stopovers: vec![aerodrome.clone()],
            departure_time: 1,
            segment_departure_time: 1,
            arrival_time: None,
            state: FlightState::Scheduled,
            interest_score: 0.0,
        });

        let cmd = ScheduleFlightCommand {
            flight_id: ScheduleFlightCommand::generate_id(),
            airplane: airplane.clone(),
            origin_aerodrome: aerodrome.clone(),
            stopovers: vec![aerodrome.clone()],
            departure_time: 2,
            interest_score: 0.0,
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<ScheduleFlightError>().unwrap();
                assert!(matches!(*err, ScheduleFlightError::AirplaneInUse));
            }
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_schedule_flight_distance_beyond_range() {
        let mut environment = Environment::default();

        let airplane = AirPlane::default();
        let origin_aerodrome = Aerodrome::default();
        let mut destination_aerodrome = Aerodrome::default();
        // Change latitude and longitude so that the distance exceeds the airplane's range
        destination_aerodrome.lat += 50.0;
        destination_aerodrome.lon += 50.0;

        let cmd = ScheduleFlightCommand {
            flight_id: ScheduleFlightCommand::generate_id(),
            airplane: airplane.clone(),
            origin_aerodrome: origin_aerodrome.clone(),
            stopovers: vec![destination_aerodrome.clone()],
            departure_time: 1,
            interest_score: 0.0,
        };

        match cmd.execute(&mut environment) {
            Err(e) => {
                let err = e.downcast::<ScheduleFlightError>().unwrap();
                assert!(matches!(*err, ScheduleFlightError::DistanceBeyondRange));
            }
            _ => panic!("Expected an error"),
        }
    }
}
