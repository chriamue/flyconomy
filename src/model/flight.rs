use crate::model::{Aerodrome, AirPlane};
use geo::{algorithm::vincenty_distance::VincentyDistance, Point};

const PROFIT_PER_KILOMETER: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct Flight {
    pub flight_id: u64,
    pub airplane: AirPlane,
    pub origin_aerodrome: Aerodrome,
    pub destination_aerodrome: Aerodrome,
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

        let profit = distance_in_kilometers * PROFIT_PER_KILOMETER;

        profit as f32
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
            airplane,
            origin_aerodrome,
            destination_aerodrome,
        };

        let profit = flight.calculate_profit();
        assert!((profit - 560.0).abs() < 1.0);

        let distance = flight.calculate_distance();
        assert!((distance - 559.0).abs() < 10.0); // allow some margin for precision errors
    }
}
