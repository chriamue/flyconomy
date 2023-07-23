use chrono::{TimeZone, Utc};

use crate::model::Timestamp;

const START_OF_2000: i64 = 946684800000;

pub fn timestamp_to_calendar_string(timestamp: Timestamp) -> String {
    let timestamp: i64 = START_OF_2000 + timestamp as i64;
    let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();
    datetime.format("%m-%d %H:%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_calendar_string() {
        let timestamp: Timestamp = 0;
        let result = timestamp_to_calendar_string(timestamp);
        assert_eq!(result, "01-01 00:00");
    }
}
