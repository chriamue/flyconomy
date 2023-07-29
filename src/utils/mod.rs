use chrono::{TimeZone, Utc};
use strsim::normalized_levenshtein;

use crate::model::{Aerodrome, Timestamp};

const START_OF_2000: i64 = 946684800000;

pub fn timestamp_to_calendar_string(timestamp: Timestamp) -> String {
    let timestamp: i64 = START_OF_2000 + timestamp as i64;
    let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();
    datetime.format("%m-%d %H:%M").to_string()
}

pub fn find_aerodrome_by_code(aerodromes: &[Aerodrome], code: &str) -> Option<Aerodrome> {
    aerodromes
        .iter()
        .find(|&aerodrome| aerodrome.code == code)
        .cloned()
}

pub fn find_best_fit_aerodrome_by_name_or_code(
    aerodromes: &[Aerodrome],
    input: &str,
) -> Option<Aerodrome> {
    aerodromes
        .iter()
        .max_by(|a, b| {
            let score_a_name = normalized_levenshtein(&a.name, input);
            let score_a_code = normalized_levenshtein(&a.code, input);
            let score_b_name = normalized_levenshtein(&b.name, input);
            let score_b_code = normalized_levenshtein(&b.code, input);

            let score_a = score_a_name.max(score_a_code);
            let score_b = score_b_name.max(score_b_code);

            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

pub fn filter_and_prioritize_aerodromes(aerodromes: &[Aerodrome], input: &str) -> Vec<Aerodrome> {
    let best_fit = find_best_fit_aerodrome_by_name_or_code(aerodromes, input);

    let filtered_aerodromes: Vec<Aerodrome> = aerodromes
        .iter()
        .filter(|a| {
            a.name.to_lowercase().contains(&input.to_lowercase())
                || a.code.to_lowercase().contains(&input.to_lowercase())
        })
        .cloned()
        .collect();

    match best_fit {
        Some(aero) => {
            let mut result = vec![aero];
            result.extend(filtered_aerodromes);
            result
        }
        None => filtered_aerodromes,
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{StringBasedWorldData, WorldDataGateway};

    use super::*;

    #[test]
    fn test_timestamp_to_calendar_string() {
        let timestamp: Timestamp = 0;
        let result = timestamp_to_calendar_string(timestamp);
        assert_eq!(result, "01-01 00:00");
    }

    #[test]
    fn test_find_frankfurt_in_world_data() {
        let world_data = StringBasedWorldData::default();
        let aerodromes = world_data.aerodromes();

        let frankfurt = find_aerodrome_by_code(aerodromes, "FRA");

        assert!(frankfurt.is_none());
        let frankfurt = find_aerodrome_by_code(aerodromes, "FRA/EDDF");

        assert!(frankfurt.is_some());
        assert_eq!(
            frankfurt.unwrap().name,
            "Frankfurt am Main Airport",
            "Frankfurt"
        );
    }

    #[test]
    fn test_find_best_fit_aerodrome_by_name_or_code() {
        let world_data = StringBasedWorldData::default();
        let aerodromes = world_data.aerodromes();

        let frankfurt = find_best_fit_aerodrome_by_name_or_code(aerodromes, "Frankfurt Main");
        assert!(frankfurt.is_some());
        assert_eq!(
            frankfurt.unwrap().code,
            "FRA/EDDF",
            "Expected Frankfurt Airport"
        );

        let lax = find_best_fit_aerodrome_by_name_or_code(aerodromes, "Los Angeles International");
        assert!(lax.is_some());
        assert_eq!(lax.unwrap().code, "LAX/KLAX", "Expected LAX Airport");
    }

    #[test]
    fn test_filter_and_prioritize_aerodromes() {
        let aerodromes = vec![
            Aerodrome {
                name: "Airport Alpha".to_string(),
                code: "AAP".to_string(),
                ..Default::default()
            },
            Aerodrome {
                name: "Airport Bravo".to_string(),
                code: "BBP".to_string(),
                ..Default::default()
            },
            Aerodrome {
                name: "Airport Charlie".to_string(),
                code: "CCP".to_string(),
                ..Default::default()
            },
        ];

        let input = "bravo";
        let results = filter_and_prioritize_aerodromes(&aerodromes, input);

        assert_eq!(
            results[0].name, "Airport Bravo",
            "The first result should be the best match"
        );
        assert_eq!(
            results.len(),
            2,
            "There should be two instances of 'Airport Bravo'"
        );
        assert_eq!(
            results[0].name, results[1].name,
            "Both instances should have the same name"
        );
    }
}
