use super::Aerodrome;

#[derive(Debug, Clone)]
pub struct Base {
    pub id: u64,
    pub aerodrome: Aerodrome,
    pub airplane_ids: Vec<u64>,
}
