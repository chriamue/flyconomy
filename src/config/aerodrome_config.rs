use std::collections::HashMap;

use csv::{ByteRecord, ReaderBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::Aerodrome;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AerodromeConfig(pub Value);

pub fn parse_airport_csv(input: &str) -> Vec<Aerodrome> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_reader(input.as_bytes());

    let mut aerodromes = Vec::new();
    let mut record = ByteRecord::new();

    while rdr.read_byte_record(&mut record).unwrap() {
        let id = std::str::from_utf8(record.get(0).unwrap())
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let name = std::str::from_utf8(record.get(1).unwrap())
            .unwrap()
            .to_string();
        let lat = std::str::from_utf8(record.get(6).unwrap())
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let lon = std::str::from_utf8(record.get(7).unwrap())
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let iata = std::str::from_utf8(record.get(4).unwrap()).unwrap();
        let icao = std::str::from_utf8(record.get(5).unwrap()).unwrap();

        let code = format!("{}/{}", iata, icao);

        let aerodrome = Aerodrome {
            id,
            lat,
            lon,
            name,
            code,
            interest_score: 1.0,
            passengers: None,
        };
        aerodromes.push(aerodrome);
    }
    aerodromes
}

pub fn parse_passengers_csv(input: &str) -> HashMap<String, u64> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(input.as_bytes());

    let mut passengers = HashMap::new();
    let mut record = ByteRecord::new();

    while rdr.read_byte_record(&mut record).unwrap() {
        let code = std::str::from_utf8(record.get(4).unwrap()).unwrap();
        let count = std::str::from_utf8(record.get(5).unwrap())
            .unwrap()
            .parse::<u64>();
        match count {
            Ok(count) => passengers.insert(code.to_string(), count),
            Err(_) => passengers.insert(code.to_string(), 0),
        };
    }
    passengers
}

pub fn load_airports(airports_csv: &str, passengers_csv: &str) -> Vec<Aerodrome> {
    let mut aerodromes = parse_airport_csv(airports_csv);
    let passengers = parse_passengers_csv(passengers_csv);

    for aerodrome in aerodromes.iter_mut() {
        let count = passengers.get(&aerodrome.code);
        aerodrome.passengers = count.copied();
    }

    aerodromes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let csv_data = include_str!("../../assets/airports.dat");

        let aerodromes = parse_airport_csv(&csv_data);

        assert_eq!(aerodromes.len(), 7698);

        assert_eq!(aerodromes[0].id, 1);
        assert_eq!(aerodromes[0].name, "Goroka Airport");
        assert_eq!(aerodromes[0].lat, -6.081689834590001);
        assert_eq!(aerodromes[0].lon, 145.391998291);
        assert_eq!(aerodromes[0].code, "GKA/AYGA");

        assert_eq!(aerodromes[1].id, 2);
        assert_eq!(aerodromes[1].name, "Madang Airport");
        assert_eq!(aerodromes[1].lat, -5.20707988739);
        assert_eq!(aerodromes[1].lon, 145.789001465);
    }

    #[test]
    fn test_parse_passengers_csv() {
        let csv_data = include_str!("../../assets/passengers.csv");

        let passengers = parse_passengers_csv(&csv_data);

        assert_eq!(passengers.len(), 69);

        assert_eq!(passengers.get("GKA/AYGA"), None);
        assert_eq!(passengers.get("HND/RJTT"), Some(&79699762));
    }
}
