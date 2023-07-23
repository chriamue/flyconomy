use chrono::{TimeZone, Utc};

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
}
