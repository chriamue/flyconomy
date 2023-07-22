use crate::{
    algorithms::calculate_interest_score,
    config::{load_airports, parse_world_heritage_site_csv, PlanesConfig},
};

use super::{Aerodrome, PlaneType, WorldHeritageSite};

pub trait WorldDataGateway: Sync + Send {
    fn reload(&mut self) -> Result<(), String>;
    fn aerodromes(&self) -> &Vec<Aerodrome>;
    fn plane_types(&self) -> &Vec<PlaneType>;
    fn world_heritage_sites(&self) -> &Vec<WorldHeritageSite>;
}

pub struct StringBasedWorldData {
    airports_csv: String,
    passengers_csv: String,
    world_heritage_sites_csv: String,
    planes_yaml: String,
    aerodromes: Vec<Aerodrome>,
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

        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.aerodromes.par_iter_mut().for_each(|aerodrome| {
                aerodrome.interest_score = calculate_interest_score(
                    aerodrome.lat,
                    aerodrome.lon,
                    &heritage_sites,
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
                    &heritage_sites,
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
            passengers_csv: String::from(include_str!("../../../assets/passengers.csv")),
            world_heritage_sites_csv: String::from(include_str!(
                "../../../assets/whc-sites-2019.csv"
            )),
            planes_yaml: String::from(include_str!("../../../assets/planes.yaml")),
            aerodromes: Vec::new(),
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

    fn world_heritage_sites(&self) -> &Vec<WorldHeritageSite> {
        &self.world_heritage_sites
    }

    fn plane_types(&self) -> &Vec<PlaneType> {
        &self.plane_types
    }

    fn reload(&mut self) -> Result<(), String> {
        self.aerodromes = load_airports(&self.airports_csv, &self.passengers_csv);
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
        assert_eq!(data.world_heritage_sites.len(), 1121);
        assert_eq!(data.plane_types.len(), 3);
    }
}
