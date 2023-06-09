use std::collections::HashMap;

use serde::{ser::Error, Deserialize, Serialize};
use serde_json::Value;

use crate::model::Aerodrome;

pub struct OverpassImporter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub id: i64,
    pub tags: Option<HashMap<String, String>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl Element {
    pub fn from_json(json_value: &Value) -> Result<Vec<Element>, serde_json::Error> {
        let railway_elements = json_value["elements"]
            .as_array()
            .ok_or_else(|| serde_json::Error::custom("Elements parsing error"))?
            .iter()
            .filter_map(|elem| serde_json::from_value::<Element>(elem.clone()).ok())
            .collect::<Vec<Element>>();
        Ok(railway_elements)
    }
}

impl From<Element> for Aerodrome {
    fn from(element: Element) -> Self {
        let tags = element.tags.unwrap();
        let name = tags.get("name").unwrap().to_string();
        let id = element.id;
        let lat = element.lat.unwrap();
        let lon = element.lon.unwrap();
        Aerodrome { id, lat, lon, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_from_json() {
        let json = r#"
        {
            "elements": [
                {
                    "type": "node",
                    "id": 37622337,
                    "lat": 48.1802843,
                    "lon": 10.7090037,
                    "tags": {
                      "aeroway": "aerodrome",
                      "contact:pobox": "1106",
                      "contact:postcode": "86826",
                      "ele": "550",
                      "icao": "EDNS",
                      "name": "Sportflugplatz Schwabmünchen",
                      "type": "private",
                      "website": "http://www.lsv-schwabmuenchen.de/",
                      "wikidata": "Q12694915"
                    }
                  },
                  {
                    "type": "node",
                    "id": 42042357,
                    "lat": 48.6373219,
                    "lon": 8.8112267,
                    "tags": {
                      "addr:country": "DE",
                      "addr:postcode": "75392",
                      "aerodrome": "gliding",
                      "aeroway": "aerodrome",
                      "name": "Segelflugplatz Deckenpfronn",
                      "website": "https://www.fsv-sindelfingen-ev.de/index.php?id=11",
                      "wikidata": "Q1725140",
                      "wikipedia": "de:Segelfluggelände Deckenpfronn-Egelsee"
                    }
                  }
            ]
        }
        "#;
        let json_value: Value = serde_json::from_str(json).unwrap();
        let elements = Element::from_json(&json_value).unwrap();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0].id, 37622337);
        assert_eq!(elements[0].lat, Some(48.1802843));
        assert_eq!(elements[0].lon, Some(10.7090037));
        assert!(elements[0].tags.clone().unwrap().contains_key("name"));
        assert_eq!(elements[1].id, 42042357);
        assert_eq!(elements[1].lat, Some(48.6373219));
        assert_eq!(elements[1].lon, Some(8.8112267));
        assert_eq!(
            *elements[1].tags.clone().unwrap().get("name").unwrap(),
            "Segelflugplatz Deckenpfronn".to_string()
        );
    }

    #[test]
    fn test_element_from_file() {
        let json = include_bytes!("../../assets/german.aerodromes.json");
        let json_value: Value = serde_json::from_slice(json).unwrap();
        let elements = Element::from_json(&json_value).unwrap();
        assert_eq!(elements.len(), 136);
    }

    #[test]
    fn test_aerodromes_from_elements() {
        let json = include_bytes!("../../assets/german.aerodromes.json");
        let json_value: Value = serde_json::from_slice(json).unwrap();
        let elements = Element::from_json(&json_value).unwrap();
        let aerodromes: Vec<Aerodrome> = elements.into_iter().map(|e| Aerodrome::from(e)).collect();
        assert_eq!(aerodromes.len(), 136);
        assert_eq!(aerodromes[0].id, 37622337);
        assert_eq!(aerodromes[0].lat, 48.1802843);
        assert_eq!(aerodromes[0].lon, 10.7090037);
        assert_eq!(aerodromes[0].name, "Sportflugplatz Schwabmünchen");
        assert_eq!(aerodromes[1].id, 42042357);
        assert_eq!(aerodromes[1].lat, 48.6373219);
        assert_eq!(aerodromes[1].lon, 8.8112267);
        assert_eq!(aerodromes[1].name, "Segelflugplatz Deckenpfronn");
    }
}
