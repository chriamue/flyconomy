use crate::{
    algorithms::calculate_interest_score,
    config::{load_airports, parse_attractions_csv, parse_world_heritage_site_csv, PlanesConfig},
};

use super::{Aerodrome, Attraction, PlaneType, WorldHeritageSite};

pub trait WorldDataGateway: Sync + Send {
    fn reload(&mut self) -> Result<(), String>;
    fn aerodromes(&self) -> &Vec<Aerodrome>;
    fn plane_types(&self) -> &Vec<PlaneType>;
    fn world_heritage_sites(&self) -> &Vec<WorldHeritageSite>;
    fn attractions(&self) -> &Vec<Attraction>;
}

pub struct StringBasedWorldData {
    airports_csv: String,
    passengers_csv: String,
    attractions_csv: String,
    world_heritage_sites_csv: String,
    planes_yaml: String,
    aerodromes: Vec<Aerodrome>,
    attractions: Vec<Attraction>,
    world_heritage_sites: Vec<WorldHeritageSite>,
    plane_types: Vec<PlaneType>,
}

impl StringBasedWorldData {
    pub fn calculate_aerodromes_interest_scores(&mut self) {
        let heritage_sites: Vec<(f64, f64, f64)> = self
            .world_heritage_sites()
            .iter()
            .map(|site| (site.lat, site.lon, 1.0f64))
            .collect();

        let mut points_of_interest = self
            .attractions()
            .iter()
            .map(|attraction| (attraction.lat, attraction.lon, 1.0f64))
            .collect::<Vec<(f64, f64, f64)>>();
        points_of_interest.extend(heritage_sites);

        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.aerodromes.par_iter_mut().for_each(|aerodrome| {
                aerodrome.interest_score = calculate_interest_score(
                    aerodrome.lat,
                    aerodrome.lon,
                    &interest_places,
                    250_000.0,
                );
            });
        }

        #[cfg(not(feature = "rayon"))]
        {
            self.aerodromes.iter_mut().for_each(|aerodrome| {
                aerodrome.interest_score = calculate_interest_score(
                    aerodrome.lat,
                    aerodrome.lon,
                    &points_of_interest,
                    250_000.0,
                );
            });
        }
    }
}

impl Default for StringBasedWorldData {
    fn default() -> Self {
        let mut data = Self {
            airports_csv: String::from(include_str!("../../../assets/airports.dat")),
            attractions_csv: String::from(include_str!("../../../assets/attractions.csv")),
            passengers_csv: String::from(include_str!("../../../assets/passengers.csv")),
            world_heritage_sites_csv: String::from(include_str!(
                "../../../assets/whc-sites-2019.csv"
            )),
            planes_yaml: String::from(include_str!("../../../assets/planes.yaml")),
            aerodromes: Vec::new(),
            attractions: Vec::new(),
            world_heritage_sites: Vec::new(),
            plane_types: Vec::new(),
        };
        data.reload().unwrap();
        data
    }
}

impl WorldDataGateway for StringBasedWorldData {
    fn aerodromes(&self) -> &Vec<Aerodrome> {
        &self.aerodromes
    }

    fn attractions(&self) -> &Vec<Attraction> {
        &self.attractions
    }

    fn world_heritage_sites(&self) -> &Vec<WorldHeritageSite> {
        &self.world_heritage_sites
    }

    fn plane_types(&self) -> &Vec<PlaneType> {
        &self.plane_types
    }

    fn reload(&mut self) -> Result<(), String> {
        self.aerodromes = load_airports(&self.airports_csv, &self.passengers_csv);
        self.attractions = parse_attractions_csv(&self.attractions_csv);
        self.world_heritage_sites = parse_world_heritage_site_csv(&self.world_heritage_sites_csv);
        self.calculate_aerodromes_interest_scores();
        let planes: PlanesConfig = serde_yaml::from_str(&self.planes_yaml).unwrap();
        self.plane_types = planes.planes;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        let data = StringBasedWorldData::default();

        assert_eq!(data.aerodromes.len(), 7698);
        assert!(data.attractions.len() > 4);
        assert_eq!(data.world_heritage_sites.len(), 1121);
        assert_eq!(data.plane_types.len(), 3);
    }
}
