#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(not(feature = "rayon"))]
use std::slice::Iter;

use super::Environment;
use crate::model::Flight;

impl Environment {
    #[cfg(not(feature = "rayon"))]
    pub fn iter_flights<'a>(&'a self) -> Iter<'a, Flight> {
        self.flights.iter()
    }

    #[cfg(feature = "rayon")]
    pub fn iter_flights<'a>(&'a self) -> rayon::slice::Iter<'a, Flight> {
        self.flights.par_iter()
    }
}
