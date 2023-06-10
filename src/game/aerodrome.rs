use bevy::prelude::{
    shape, App, Assets, Color, Commands, Component, Mesh, PbrBundle, Res, ResMut, Resource,
    StandardMaterial, Transform, Vec3,
};

use crate::{config::AerodromeConfig, model::Aerodrome, overpass_importer::Element};

use super::ConfigResource;

const EARTH_RADIUS: f64 = 6_371_000.0;
const SIMULATION_EARTH_RADIUS: f64 = 1.0;
const SCALE_FACTOR: f64 = SIMULATION_EARTH_RADIUS / EARTH_RADIUS;

pub fn add_aerodrome_systems_to_app(app: &mut App) {
    app.insert_resource(AerodromeSystem { setup_done: false })
        .add_system(setup);
}

#[derive(Resource)]
struct AerodromeSystem {
    setup_done: bool,
}

#[derive(Component)]
pub struct AerodromeComponent(pub Aerodrome);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config_resource: Res<ConfigResource>,
    mut aerodrome_system: ResMut<AerodromeSystem>,
) {
    if !aerodrome_system.setup_done {
        if let Some(aerodromes_config) = config_resource.aerodrome_config.as_ref() {
            if let Ok(elements) = Element::from_json(&aerodromes_config.0) {
                let aerodromes: Vec<Aerodrome> =
                    elements.into_iter().map(Aerodrome::from).collect();

                for aerodrome in aerodromes {
                    let position = project(aerodrome.lat, aerodrome.lon);
                    let mesh_handle = meshes.add(
                        Mesh::try_from(shape::Icosphere {
                            radius: 10_000.0 * SCALE_FACTOR as f32,
                            subdivisions: 4,
                        })
                        .unwrap(),
                    );
                    let material_handle = materials.add(StandardMaterial {
                        base_color: Color::rgb(0.0, 0.0, 1.0),
                        ..Default::default()
                    });

                    println!("Adding aerodrome: {:?}", aerodrome);
                    println!("Adding aerodrome at: {:?}", position);

                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: material_handle,
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .insert(AerodromeComponent(aerodrome));
                }
            }
            aerodrome_system.setup_done = true;
        }
    }
}

pub fn project(lon: f64, lat: f64) -> Vec3 {
    let lat_rad = -lat.to_radians();
    let lon_rad = lon.to_radians();
    let x = EARTH_RADIUS * lat_rad.cos() * lon_rad.cos();
    let y = EARTH_RADIUS * lat_rad.cos() * lon_rad.sin();
    let z = EARTH_RADIUS * lat_rad.sin();
    Vec3::new(x as f32, y as f32, z as f32) * SCALE_FACTOR as f32
}
