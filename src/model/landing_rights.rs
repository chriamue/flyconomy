use super::Aerodrome;

#[derive(Debug, Clone)]
pub struct LandingRights {
    pub id: u64,
    pub aerodrome: Aerodrome,
}
