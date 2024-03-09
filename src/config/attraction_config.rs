use csv::{ByteRecord, ReaderBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::Attraction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttractionConfig(pub Value);

pub fn parse_attractions_csv(input: &str) -> Vec<Attraction> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(input.as_bytes());

    let mut sites = Vec::new();
    let mut record = ByteRecord::new();

    while rdr.read_byte_record(&mut record).unwrap() {
        let id = std::str::from_utf8(record.get(0).unwrap())
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let name = std::str::from_utf8(record.get(1).unwrap())
            .unwrap()
            .to_string();
        let description = std::str::from_utf8(record.get(4).unwrap())
            .unwrap()
            .to_string();
        let lat = std::str::from_utf8(record.get(2).unwrap())
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let lon = std::str::from_utf8(record.get(3).unwrap())
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let site = Attraction {
            id,
            lat,
            lon,
            name,
            description,
        };
        sites.push(site);
    }
    sites
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let csv_data = include_str!("../../assets/attractions.csv");

        let sites = parse_attractions_csv(&csv_data);

        assert!(sites.len() > 4);
    }
}
