use crate::model::{Aerodrome, AirPlane};
use geo::{algorithm::vincenty_distance::VincentyDistance, Point};

use super::Timestamp;

const PROFIT_PER_KILOMETER: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct Flight {
    pub flight_id: u64,
    pub airplane: AirPlane,
    pub origin_aerodrome: Aerodrome,
    pub stopovers: Vec<Aerodrome>,
    pub departure_time: Timestamp,
    pub segment_departure_time: Timestamp,
    pub arrival_time: Option<Timestamp>,
    pub state: FlightState,
    pub interest_score: f64,
}

impl Default for Flight {
    fn default() -> Self {
        Self {
            flight_id: 0,
            airplane: AirPlane::default(),
            origin_aerodrome: Aerodrome::frankfurt(),
            stopovers: vec![Aerodrome::paris()],
            departure_time: 0,
            segment_departure_time: 0,
            arrival_time: None,
            state: FlightState::Scheduled,
            interest_score: 0.0,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum FlightState {
    #[default]
    Scheduled,
    EnRoute {
        next_stopover_index: usize,
    },
    Landed {
        next_stopover_index: usize,
    },
    Finished,
}

impl Flight {
    #[deprecated]
    pub fn calculate_distance(&self) -> f64 {
        let origin_point = Point::new(self.origin_aerodrome.lon, self.origin_aerodrome.lat);
        let destination_point = Point::new(self.stopovers[0].lon, self.stopovers[0].lat);
        let distance_in_meters = origin_point.vincenty_distance(&destination_point).unwrap();
        distance_in_meters / 1000.0
    }

    pub fn calculate_booked_seats(&self) -> u32 {
        let seats = self.airplane.plane_type.seats as f64;
        let interest_score_5 = 1.0 + 4.0 * self.interest_score;
        let booked_seats = seats * interest_score_5 / 5.0;
        booked_seats.round() as u32
    }

    pub fn calculate_profit(&self) -> f64 {
        let distance_in_kilometers = self.calculate_total_distance();
        let seats = self.calculate_booked_seats() as f64;
        let profit = distance_in_kilometers * PROFIT_PER_KILOMETER * seats;

        profit
    }

    pub fn update_state(&mut self, current_time: Timestamp) {
        match self.state {
            FlightState::Scheduled if current_time >= self.departure_time => {
                self.state = FlightState::EnRoute {
                    next_stopover_index: 0,
                };
                self.segment_departure_time = self.departure_time;
                self.update_arrival_time();
            }
            FlightState::EnRoute {
                next_stopover_index,
            } if Some(current_time) >= self.arrival_time => {
                if next_stopover_index < self.stopovers.len() {
                    self.state = FlightState::Landed {
                        next_stopover_index,
                    };
                    self.segment_departure_time = current_time + 30 * 60 * 1000;
                // 30 minutes
                } else {
                    self.state = FlightState::Finished;
                }
            }
            FlightState::Landed {
                next_stopover_index,
            } if current_time >= self.segment_departure_time => {
                self.state = FlightState::EnRoute {
                    next_stopover_index: next_stopover_index + 1,
                };
                self.update_arrival_time();
            }
            _ => {}
        }
    }

    fn update_arrival_time(&mut self) {
        if let Some(next_aerodrome) = self.current_destination() {
            let distance =
                Flight::calculate_distance_between(&self.current_origin(), &next_aerodrome);
            let speed = self.airplane.plane_type.speed;
            self.arrival_time = Some(
                self.segment_departure_time + (distance / speed as f64 * 3_600_000.0) as Timestamp,
            );
            // assuming speed in km/h
        }
    }

    pub fn estimate_current_position(&self, timestamp: Timestamp) -> Option<(f64, f64)> {
        match &self.state {
            FlightState::Scheduled => None,
            FlightState::EnRoute { .. } | FlightState::Landed { .. } => {
                let current_origin = self.current_origin();
                if let (Some(current_destination), Some(arrival_time)) =
                    (self.current_destination(), self.arrival_time)
                {
                    if timestamp >= self.segment_departure_time && timestamp <= arrival_time {
                        let total_flight_time = arrival_time - self.segment_departure_time;
                        let elapsed_time = timestamp - self.segment_departure_time;

                        // Calculate the fraction of the trip completed
                        let fraction = elapsed_time as f64 / total_flight_time as f64;

                        let start_coords = (current_origin.lat, current_origin.lon);
                        let end_coords = (current_destination.lat, current_destination.lon);

                        // Interpolate the current coordinates
                        let current_latitude =
                            start_coords.0 + fraction * (end_coords.0 - start_coords.0);
                        let current_longitude =
                            start_coords.1 + fraction * (end_coords.1 - start_coords.1);

                        Some((current_latitude, current_longitude))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            FlightState::Finished => Some((self.origin_aerodrome.lat, self.origin_aerodrome.lon)),
        }
    }

    pub fn calculate_total_distance(&self) -> f64 {
        self.distance_between_aerodromes(&self.get_complete_itinerary())
    }

    fn distance_between_aerodromes(&self, aerodromes: &[Aerodrome]) -> f64 {
        aerodromes
            .windows(2)
            .map(|aerodromes| Flight::calculate_distance_between(&aerodromes[0], &aerodromes[1]))
            .sum()
    }

    pub fn calculate_distance_between(aerodrome1: &Aerodrome, aerodrome2: &Aerodrome) -> f64 {
        let point1 = Point::new(aerodrome1.lon, aerodrome1.lat);
        let point2 = Point::new(aerodrome2.lon, aerodrome2.lat);
        let distance_in_meters = point1.vincenty_distance(&point2).unwrap();
        distance_in_meters / 1000.0
    }

    pub fn is_plane_range_sufficient(&self) -> bool {
        let plane_range = self.airplane.plane_type.range as f64;
        self.get_complete_itinerary().windows(2).all(|aerodromes| {
            let distance = Flight::calculate_distance_between(&aerodromes[0], &aerodromes[1]);
            distance <= plane_range
        })
    }

    fn get_complete_itinerary(&self) -> Vec<Aerodrome> {
        let mut itinerary = self.stopovers.clone();
        itinerary.insert(0, self.origin_aerodrome.clone()); // Inserting origin at start
        itinerary.push(self.origin_aerodrome.clone()); // Adding origin at end for round trip
        itinerary
    }

    pub fn current_origin(&self) -> Aerodrome {
        match &self.state {
            FlightState::Scheduled => self.origin_aerodrome.clone(),
            FlightState::EnRoute {
                next_stopover_index,
            }
            | FlightState::Landed {
                next_stopover_index,
            } => {
                if *next_stopover_index > 0 {
                    self.stopovers[*next_stopover_index - 1].clone()
                } else {
                    self.origin_aerodrome.clone()
                }
            }
            FlightState::Finished => self
                .stopovers
                .last()
                .unwrap_or(&self.origin_aerodrome)
                .clone(),
        }
    }

    pub fn current_destination(&self) -> Option<Aerodrome> {
        match &self.state {
            FlightState::Scheduled => self.stopovers.get(0).cloned(),
            FlightState::EnRoute {
                next_stopover_index,
            }
            | FlightState::Landed {
                next_stopover_index,
            } => {
                if *next_stopover_index < self.stopovers.len() {
                    self.stopovers.get(*next_stopover_index).cloned()
                } else {
                    Some(self.origin_aerodrome.clone())
                }
            }
            FlightState::Finished => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_profit() {
        let airplane = AirPlane::default();

        let origin_aerodrome = Aerodrome {
            id: 1,
            lat: 37.7749, // Coordinates of San Francisco
            lon: -122.4194,
            name: "San Francisco International Airport".to_string(),
            code: "SFO/KSFO".to_string(),
            passengers: None,
        };

        let destination_aerodrome = Aerodrome {
            id: 2,
            lat: 34.0522, // Coordinates of Los Angeles
            lon: -118.2437,
            name: "Los Angeles International Airport".to_string(),
            code: "LAX/KLAX".to_string(),
            passengers: None,
        };

        let flight = Flight {
            flight_id: 1,
            airplane: airplane.clone(),
            origin_aerodrome,
            departure_time: 0,
            segment_departure_time: 0,
            arrival_time: None,
            state: Default::default(),
            interest_score: 1.0,
            stopovers: vec![destination_aerodrome],
        };

        let distance = flight.calculate_total_distance();
        let expected_profit = distance * airplane.plane_type.seats as f64;
        let profit = flight.calculate_profit();
        assert!((profit - expected_profit).abs() < 1.0);
    }

    #[test]
    fn test_calculate_distance() {
        let frankfurt = Aerodrome::frankfurt();
        let paris = Aerodrome::paris();

        let distance = Flight::calculate_distance_between(&frankfurt, &paris);
        assert!((distance - 450.0).abs() < 1.0);
    }
}
