use crate::model::{Aerodrome, AirPlane};
use geo::{algorithm::vincenty_distance::VincentyDistance, Point};

const PROFIT_PER_KILOMETER: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct Flight {
    pub flight_id: u64,
    pub airplane: AirPlane,
    pub origin_aerodrome: Aerodrome,
    pub destination_aerodrome: Aerodrome,
    pub departure_time: u64,
    pub arrival_time: Option<u64>,
    pub state: FlightState,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum FlightState {
    #[default]
    Scheduled,
    EnRoute,
    Landed,
}

impl Flight {
    pub fn calculate_distance(&self) -> f64 {
        let origin_point = Point::new(self.origin_aerodrome.lon, self.origin_aerodrome.lat);
        let destination_point = Point::new(
            self.destination_aerodrome.lon,
            self.destination_aerodrome.lat,
        );

        let distance_in_meters = origin_point.vincenty_distance(&destination_point).unwrap();

        distance_in_meters / 1000.0
    }

    pub fn calculate_profit(&self) -> f32 {
        let distance_in_kilometers = self.calculate_distance();
        let seats = self.airplane.plane_type.seats as f64;
        let profit = distance_in_kilometers * PROFIT_PER_KILOMETER * seats;

        profit as f32
    }

    pub fn update_state(&mut self, current_time: u64) {
        if self.state == FlightState::Scheduled && current_time >= self.departure_time {
            self.state = FlightState::EnRoute;
            let distance = self.calculate_distance();
            let speed = self.airplane.plane_type.speed;
            self.arrival_time =
                Some(self.departure_time + (distance / speed as f64 * 3600.0) as u64);
        // assuming speed in km/h
        } else if self.state == FlightState::EnRoute && current_time >= self.arrival_time.unwrap() {
            self.state = FlightState::Landed;
        }
    }

    pub fn estimate_current_position(&self, timestamp: f64) -> Option<(f64, f64)> {
        if let Some(arrival_time) = self.arrival_time {
            if timestamp >= self.departure_time as f64 && timestamp <= arrival_time as f64 {
                let total_flight_time = arrival_time - self.departure_time;
                let elapsed_time = timestamp - self.departure_time as f64;

                // Calculate the fraction of the trip completed
                let fraction = elapsed_time / total_flight_time as f64;

                let origin_lat = self.origin_aerodrome.lat;
                let origin_lon = self.origin_aerodrome.lon;
                let dest_lat = self.destination_aerodrome.lat;
                let dest_lon = self.destination_aerodrome.lon;

                // Linear interpolation between origin and destination
                let current_lat = origin_lat + fraction * (dest_lat - origin_lat);
                let current_lon = origin_lon + fraction * (dest_lon - origin_lon);

                return Some((current_lat, current_lon));
            }
        }
        None
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
        };

        let destination_aerodrome = Aerodrome {
            id: 2,
            lat: 34.0522, // Coordinates of Los Angeles
            lon: -118.2437,
            name: "Los Angeles International Airport".to_string(),
        };

        let flight = Flight {
            flight_id: 1,
            airplane: airplane.clone(),
            origin_aerodrome,
            destination_aerodrome,
            departure_time: 0,
            arrival_time: None,
            state: Default::default(),
        };

        let distance = flight.calculate_distance();
        let expected_profit = distance * airplane.plane_type.seats as f64;
        let profit = flight.calculate_profit();
        assert!((profit - expected_profit as f32).abs() < 1.0);
    }
}
