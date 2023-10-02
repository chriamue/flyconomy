use async_trait::async_trait;

mod attraction;
pub mod web3;

pub use crate::attraction::*;
pub use crate::web3::*;

#[async_trait(?Send)]
pub trait AttractionContract {
    async fn get_all_locations(&self) -> Result<Vec<Attraction>, Box<dyn std::error::Error>>;
    async fn get_name(&self, id: u64) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_description(&self, id: u64) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_lat(&self, id: u64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn get_lon(&self, id: u64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn get_location(&self, id: u64) -> Result<(f64, f64), Box<dyn std::error::Error>>;
    async fn get_total_supply(&self) -> Result<u64, Box<dyn std::error::Error>>;
}
