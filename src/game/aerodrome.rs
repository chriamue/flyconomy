use bevy::prelude::{
    shape, App, Assets, Color, Commands, Component, Mesh, PbrBundle, Res, ResMut, Resource,
    StandardMaterial, Transform,
};

use crate::{
    game::{earth3d, projection::wgs84_to_xyz},
    model::Aerodrome,
    overpass_importer::Element,
};

use super::ConfigResource;

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
                    let position = wgs84_to_xyz(aerodrome.lat, aerodrome.lon, 0.0)
                        * earth3d::SCALE_FACTOR as f32;
                    let mesh_handle = meshes.add(
                        Mesh::try_from(shape::Icosphere {
                            radius: 10_000.0 * earth3d::SCALE_FACTOR as f32,
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
