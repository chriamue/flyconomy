use super::Aerodrome;

#[derive(Debug, Clone)]
pub struct LandingRights {
    pub id: u32,
    pub aerodrome: Aerodrome,
}
