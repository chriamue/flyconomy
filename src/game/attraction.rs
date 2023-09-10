use bevy::asset::Handle;
use bevy::prelude::{
    shape, App, Assets, Color, Commands, Component, Entity, Event, EventReader, EventWriter, Mesh,
    PbrBundle, Plugin, Query, Res, ResMut, Resource, StandardMaterial, Transform, Update, Vec3,
    With,
};
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_eventlistener::prelude::On;
use bevy_mod_picking::prelude::Pointer;
use bevy_mod_picking::{
    prelude::{Click, RaycastPickTarget},
    PickableBundle,
};

use crate::{
    game::{earth3d, projection::wgs84_to_xyz},
    model::Attraction,
};

use super::GameResource;

const ATTRACTION_COLOR: Color = Color::rgb(1.0, 0.6, 0.0);
const ATTRACTION_COLOR_SELECTED: Color = Color::rgb(1.0, 0.0, 0.0);

pub struct AttractionPlugin;

impl Plugin for AttractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AttractionSystem { setup_done: false })
            .add_systems(
                Update,
                (
                    setup,
                    handle_attraction_selected_event,
                    handle_selected_attraction_change_event,
                ),
            )
            .add_event::<AttractionSelectedEvent>()
            .add_event::<SelectedAttractionChangeEvent>()
            .insert_resource(SelectedAttraction::default());
    }
}

#[derive(Event)]
pub struct SelectedAttractionChangeEvent(pub Attraction);

#[derive(Resource)]
struct AttractionSystem {
    setup_done: bool,
}

#[derive(Default, Component)]
pub struct AttractionComponent(pub Attraction);

#[derive(Default, Resource, Debug)]
pub struct SelectedAttraction {
    pub attraction: Option<Attraction>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut attraction_system: ResMut<AttractionSystem>,
    game_resource: Res<GameResource>,
) {
    if !attraction_system.setup_done {
        let attractions = game_resource.simulation.world_data_gateway.attractions();
        for attraction in attractions {
            let position =
                wgs84_to_xyz(attraction.lat, attraction.lon, 0.0) * earth3d::SCALE_FACTOR as f32;
            let mesh_handle = meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 12_000.0 * earth3d::SCALE_FACTOR as f32,
                    subdivisions: 1,
                })
                .unwrap(),
            );
            let material_handle = materials.add(StandardMaterial {
                base_color: ATTRACTION_COLOR,
                ..Default::default()
            });

            commands
                .spawn((
                    PbrBundle {
                        mesh: mesh_handle,
                        material: material_handle,
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    },
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                    On::<Pointer<Click>>::send_event::<AttractionSelectedEvent>(),
                ))
                .insert(AttractionComponent(attraction.clone()));
        }
        attraction_system.setup_done = true;
    }
}

#[derive(Debug, Component, Event)]
struct AttractionSelectedEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for AttractionSelectedEvent {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self {
        Self(click_event.target)
    }
}

fn handle_attraction_selected_event(
    mut event: EventReader<AttractionSelectedEvent>,
    attraction_query: Query<(Entity, &AttractionComponent)>,
    mut ev_selected_attraction_change: EventWriter<SelectedAttractionChangeEvent>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, attraction_component)) = attraction_query.get(select_event.0) {
            ev_selected_attraction_change.send(SelectedAttractionChangeEvent(
                attraction_component.0.clone(),
            ));
        }
    }
}

fn handle_selected_attraction_change_event(
    mut event: EventReader<SelectedAttractionChangeEvent>,
    attraction_query: Query<(Entity, &AttractionComponent)>,
    mut selected_attraction: ResMut<SelectedAttraction>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_query: Query<&Handle<StandardMaterial>, With<Handle<StandardMaterial>>>,
    mut transform_query: Query<&mut Transform>,
) {
    for event in event.iter() {
        let attraction = &event.0;

        if let Some(selected_attraction) = selected_attraction.attraction.as_ref() {
            for (entity, attraction_component) in attraction_query.iter() {
                if attraction_component.0 == *selected_attraction {
                    if let Ok(material_handle) = mesh_query.get(entity) {
                        if let Some(material) = materials.get_mut(material_handle) {
                            material.base_color = ATTRACTION_COLOR;
                        }
                    }
                    if let Ok(mut transform) = transform_query.get_mut(entity) {
                        transform.scale = Vec3::splat(1.0);
                    }
                    break;
                }
            }
        }

        for (entity, attraction_component) in attraction_query.iter() {
            if attraction_component.0 == *attraction {
                selected_attraction.attraction = Some(attraction.clone());
                if let Ok(material_handle) = mesh_query.get(entity) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = ATTRACTION_COLOR_SELECTED;
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
