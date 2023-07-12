use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Aerodrome {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub code: String,
    pub passengers: Option<u64>,
}

impl Aerodrome {
    pub fn new(id: u64, lat: f64, lon: f64, name: String, code: String) -> Self {
        Self {
            id,
            lat,
            lon,
            name,
            code,
            passengers: None,
        }
    }

    pub fn frankfurt() -> Self {
        Self::new(
            340,
            50.033333,
            8.570556,
            "Frankfurt am Main Airport".to_string(),
            "FRA/EDDF".to_string(),
        )
    }

    pub fn paris() -> Self {
        Self::new(
            1382,
            49.012798,
            2.55,
            "Charles de Gaulle International Airport".to_string(),
            "CDG/LFPG".to_string(),
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::Aerodrome;

    impl Aerodrome {
        pub fn san_francisco() -> Self {
            Self::new(
                3469,
                37.61899948120117,
                -122.375,
                "San Francisco International Airport".to_string(),
                "SFO/KSFO".to_string(),
            )
        }

        pub fn new_york() -> Self {
            Self::new(
                3797,
                40.63980103,
                -73.77890015,
                "John F Kennedy International Airport".to_string(),
                "JFK/KJFK".to_string(),
            )
        }
    }
}
