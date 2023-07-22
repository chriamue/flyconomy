use geo::{algorithm::vincenty_distance::VincentyDistance, Point};

pub fn calculate_interest_score(
    lat: f64,
    lon: f64,
    points_of_interest: &Vec<(f64, f64, f64)>,
    max_distance_meters: f64,
) -> f32 {
    let point1 = Point::new(lon, lat);

    let interest_score: f64 = points_of_interest
        .iter()
        .map(|&(poi_lat, poi_lon, poi_score)| {
            let point2 = Point::new(poi_lon, poi_lat);
            let distance = point1.vincenty_distance(&point2).unwrap_or_default();

            if distance <= max_distance_meters {
                // Scale the score based on distance, now using the square of the distance.
                let distance_ratio = (distance / max_distance_meters).powi(2);
                (1.0 - distance_ratio) * poi_score
            } else {
                0.0
            }
        })
        .fold(0.0, |acc, score| acc + score)
        / 10.0;

    // Normalize the final score to lie within [0.0, 1.0] range.
    // The score may exceed 1.0 if there are multiple points of interest nearby,
    // each contributing a part of the score. We cap it at 1.0 for consistency.
    interest_score.min(1.0) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_score_no_pois() {
        let lat = 40.748817;
        let lon = -73.985428;
        let pois = vec![];
        let max_distance = 5000.0; // meters
        let score = calculate_interest_score(lat, lon, &pois, max_distance);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_interest_score_one_poi_at_max_distance() {
        let lat = 40.748817;
        let lon = -73.985428;
        let pois = vec![(40.764936, -73.980862, 1.0)]; // Central Park (approx 2km from the Statue of Liberty)
        let max_distance = 2000.0; // meters
        let score = calculate_interest_score(lat, lon, &pois, max_distance);
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_interest_score_one_poi_at_half_max_distance() {
        let lat = 40.748817;
        let lon = -73.985428;
        let pois = vec![(40.764936, -73.980862, 1.0)]; // Central Park (approx 1km from the Statue of Liberty)
        let max_distance = 5_000.0; // meters
        let score = calculate_interest_score(lat, lon, &pois, max_distance);
        assert!(score > 0.08 && score <= 1.0);
    }

    #[test]
    fn test_interest_score_multiple_pois() {
        let lat = 40.748817;
        let lon = -73.985428;
        let pois = vec![
            (40.764936, -73.980862, 1.0), // Central Park
            (40.712776, -74.005974, 0.8), // Wall Street (approx 5km from the Statue of Liberty)
        ];
        let max_distance = 5000.0; // meters
        let score = calculate_interest_score(lat, lon, &pois, max_distance);
        assert!(score > 0.1 && score <= 0.2);
    }
}
