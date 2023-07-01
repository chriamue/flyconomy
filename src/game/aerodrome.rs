use bevy::asset::Handle;
use bevy::prelude::{
    shape, App, Assets, Color, Commands, Component, Entity, EventReader, EventWriter, Mesh,
    PbrBundle, Plugin, Query, Res, ResMut, Resource, StandardMaterial, Transform, Vec3, With,
};
use bevy_mod_picking::{
    prelude::{Click, ListenedEvent, OnPointer, RaycastPickTarget},
    PickableBundle,
};

use crate::{
    game::{earth3d, projection::wgs84_to_xyz},
    model::Aerodrome,
};

use super::ConfigResource;

const AERODROME_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const AERODROME_COLOR_SELECTED: Color = Color::rgb(1.0, 0.0, 0.0);

pub struct AerodromePlugin;

impl Plugin for AerodromePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AerodromeSystem { setup_done: false })
            .add_system(setup)
            .add_event::<AerodromeSelectedEvent>()
            .add_event::<SelectedAerodromeChangeEvent>()
            .insert_resource(SelectedAerodrome::default())
            .add_system(handle_aerodrome_selected_event)
            .add_system(handle_selected_aerodrome_change_event);
    }
}

pub struct SelectedAerodromeChangeEvent(pub Aerodrome);

#[derive(Resource)]
struct AerodromeSystem {
    setup_done: bool,
}

#[derive(Default, Component)]
pub struct AerodromeComponent(pub Aerodrome);

#[derive(Default, Resource, Debug)]
pub struct SelectedAerodrome {
    pub aerodrome: Option<Aerodrome>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config_resource: Res<ConfigResource>,
    mut aerodrome_system: ResMut<AerodromeSystem>,
) {
    if !aerodrome_system.setup_done {
        if let Some(aerodromes) = config_resource.aerodromes.as_ref() {
            for aerodrome in aerodromes {
                let position =
                    wgs84_to_xyz(aerodrome.lat, aerodrome.lon, 0.0) * earth3d::SCALE_FACTOR as f32;
                let mesh_handle = meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: 12_000.0 * earth3d::SCALE_FACTOR as f32,
                        subdivisions: 1,
                    })
                    .unwrap(),
                );
                let material_handle = materials.add(StandardMaterial {
                    base_color: AERODROME_COLOR,
                    ..Default::default()
                });
                let mut transform = Transform::from_translation(position);
                match aerodrome.passengers {
                    Some(passengers) => {
                        transform.scale = Vec3::splat(1.0)
                            + Vec3::splat(1.0) * (passengers as f32 / 50_000_000.0);
                    }
                    None => {
                        transform.scale = Vec3::splat(1.0);
                    }
                }

                commands
                    .spawn((
                        PbrBundle {
                            mesh: mesh_handle,
                            material: material_handle,
                            transform,
                            ..Default::default()
                        },
                        PickableBundle::default(),
                        RaycastPickTarget::default(),
                        OnPointer::<Click>::send_event::<AerodromeSelectedEvent>(),
                    ))
                    .insert(AerodromeComponent(aerodrome.clone()));
            }
            aerodrome_system.setup_done = true;
        }
    }
}

#[derive(Debug, Component)]
struct AerodromeSelectedEvent(Entity);

impl From<ListenedEvent<Click>> for AerodromeSelectedEvent {
    fn from(click_event: ListenedEvent<Click>) -> Self {
        Self(click_event.target)
    }
}

fn handle_aerodrome_selected_event(
    mut event: EventReader<AerodromeSelectedEvent>,
    aerodrome_query: Query<(Entity, &AerodromeComponent)>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, aerodrome_component)) = aerodrome_query.get(select_event.0) {
            ev_selected_aerodrome_change
                .send(SelectedAerodromeChangeEvent(aerodrome_component.0.clone()));
        }
    }
}

fn handle_selected_aerodrome_change_event(
    mut event: EventReader<SelectedAerodromeChangeEvent>,
    aerodrome_query: Query<(Entity, &AerodromeComponent)>,
    mut selected_aerodrome: ResMut<SelectedAerodrome>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_query: Query<&Handle<StandardMaterial>, With<Handle<StandardMaterial>>>,
    mut transform_query: Query<&mut Transform>,
) {
    for event in event.iter() {
        let aerodrome = &event.0;

        if let Some(selected_aerodrome) = selected_aerodrome.aerodrome.as_ref() {
            for (entity, aerodrome_component) in aerodrome_query.iter() {
                if aerodrome_component.0 == *selected_aerodrome {
                    if let Ok(material_handle) = mesh_query.get(entity) {
                        if let Some(mut material) = materials.get_mut(material_handle) {
                            material.base_color = AERODROME_COLOR;
                        }
                    }
                    if let Ok(mut transform) = transform_query.get_mut(entity) {
                        match aerodrome.passengers {
                            Some(passengers) => {
                                transform.scale = Vec3::splat(1.0)
                                    + Vec3::splat(1.0) * (passengers as f32 / 50_000_000.0);
                            }
                            None => {
                                transform.scale = Vec3::splat(1.0);
                            }
                        }
                    }

                    break;
                }
            }
        }

        for (entity, aerodrome_component) in aerodrome_query.iter() {
            if aerodrome_component.0 == *aerodrome {
                selected_aerodrome.aerodrome = Some(aerodrome.clone());
                if let Ok(material_handle) = mesh_query.get(entity) {
                    if let Some(mut material) = materials.get_mut(material_handle) {
                        material.base_color = AERODROME_COLOR_SELECTED;
                    }
                }
                if let Ok(mut transform) = transform_query.get_mut(entity) {
                    transform.scale = Vec3::splat(2.0);
                }
                break;
            }
        }
    }
}
