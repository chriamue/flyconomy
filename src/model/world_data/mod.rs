use crate::config::{load_airports, parse_world_heritage_site_csv, PlanesConfig};

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
        let planes: PlanesConfig = serde_yaml::from_str(&self.planes_yaml).unwrap();
        self.plane_types = planes.planes;
        Ok(())
    }
}
