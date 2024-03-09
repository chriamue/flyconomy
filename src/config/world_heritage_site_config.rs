use csv::{ByteRecord, ReaderBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::WorldHeritageSite;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldHeritageSiteConfig(pub Value);

pub fn parse_world_heritage_site_csv(input: &str) -> Vec<WorldHeritageSite> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(input.as_bytes());

    let mut sites = Vec::new();
    let mut record = ByteRecord::new();

    while rdr.read_byte_record(&mut record).unwrap() {
        let id = std::str::from_utf8(record.get(4).unwrap())
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let name = std::str::from_utf8(record.get(6).unwrap())
            .unwrap()
            .to_string();
        let description = std::str::from_utf8(record.get(7).unwrap())
            .unwrap()
            .to_string();
        let lat = std::str::from_utf8(record.get(15).unwrap())
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let lon = std::str::from_utf8(record.get(14).unwrap())
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let site = WorldHeritageSite {
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
        let csv_data = include_str!("../../assets/whc-sites-2019.csv");

        let sites = parse_world_heritage_site_csv(&csv_data);

        assert_eq!(sites.len(), 1121);

        assert_eq!(sites[0].id, 208);
        assert_eq!(
            sites[0].name,
            "Cultural Landscape and Archaeological Remains of the Bamiyan Valley"
        );
        assert_eq!(sites[0].lat, 34.84694);
        assert_eq!(sites[0].lon, 67.82525);
    }
}
