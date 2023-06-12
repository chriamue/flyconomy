use super::PlaneType;

#[derive(Debug, Default, Clone)]
pub struct AirPlane {
    pub id: u64,
    pub base_id: u64,
    pub plane_type: PlaneType,
}
