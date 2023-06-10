use bevy::prelude::Vec3;

const WGS84_A: f64 = 6_378_137.0; // Semi-major axis
const WGS84_E: f64 = 0.0818191908426; // First eccentricity

/// Converts a GPS coordinate to a 3D point on a sphere.
///
/// * `lat` - Latitude in degrees.
/// * `lon` - Longitude in degrees.
///
/// # Returns
///
/// A 3D point in a Right-Handed Y-Up coordinate system
///
pub fn wgs84_to_xyz(lat: f64, lon: f64, alt: f64) -> Vec3 {
    let lat_rad = lat.to_radians();
    let lon_rad = (-lon).to_radians();
    let e2 = WGS84_E.powi(2);
    let n = WGS84_A / (1.0 - e2 * lat_rad.sin().powi(2)).sqrt();

    let x = (n + alt) * lat_rad.cos() * lon_rad.cos();
    let y = (n + alt) * lat_rad.cos() * lon_rad.sin();
    let z = ((1.0 - e2) * n + alt) * lat_rad.sin();

    Vec3::new(x as f32, z as f32, y as f32)
}
