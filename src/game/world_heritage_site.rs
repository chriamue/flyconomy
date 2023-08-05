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
    model::WorldHeritageSite,
};

use super::GameResource;

const SITE_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);
const SITE_COLOR_SELECTED: Color = Color::rgb(1.0, 0.0, 0.0);

pub struct WorldHeritageSitePlugin;

impl Plugin for WorldHeritageSitePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldHeritageSiteSystem { setup_done: false })
            .add_systems(
                Update,
                (
                    setup,
                    handle_world_heritage_site_selected_event,
                    handle_selected_world_heritage_site_change_event,
                ),
            )
            .add_event::<WorldHeritageSiteSelectedEvent>()
            .add_event::<SelectedWorldHeritageSiteChangeEvent>()
            .insert_resource(SelectedWorldHeritageSite::default());
    }
}

#[derive(Event)]
pub struct SelectedWorldHeritageSiteChangeEvent(pub WorldHeritageSite);

#[derive(Resource)]
struct WorldHeritageSiteSystem {
    setup_done: bool,
}

#[derive(Default, Component)]
pub struct WorldHeritageSiteComponent(pub WorldHeritageSite);

#[derive(Default, Resource, Debug)]
pub struct SelectedWorldHeritageSite {
    pub site: Option<WorldHeritageSite>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut site_system: ResMut<WorldHeritageSiteSystem>,
    game_resource: Res<GameResource>,
) {
    if !site_system.setup_done {
        let sites = game_resource
            .simulation
            .world_data_gateway
            .world_heritage_sites();
        for site in sites {
            let position = wgs84_to_xyz(site.lat, site.lon, 0.0) * earth3d::SCALE_FACTOR as f32;
            let mesh_handle = meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 12_000.0 * earth3d::SCALE_FACTOR as f32,
                    subdivisions: 1,
                })
                .unwrap(),
            );
            let material_handle = materials.add(StandardMaterial {
                base_color: SITE_COLOR,
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
                    On::<Pointer<Click>>::send_event::<WorldHeritageSiteSelectedEvent>(),
                ))
                .insert(WorldHeritageSiteComponent(site.clone()));
        }
        site_system.setup_done = true;
    }
}

#[derive(Debug, Component, Event)]
struct WorldHeritageSiteSelectedEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for WorldHeritageSiteSelectedEvent {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self {
        Self(click_event.target)
    }
}

fn handle_world_heritage_site_selected_event(
    mut event: EventReader<WorldHeritageSiteSelectedEvent>,
    site_query: Query<(Entity, &WorldHeritageSiteComponent)>,
    mut ev_selected_site_change: EventWriter<SelectedWorldHeritageSiteChangeEvent>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, site_component)) = site_query.get(select_event.0) {
            ev_selected_site_change.send(SelectedWorldHeritageSiteChangeEvent(
                site_component.0.clone(),
            ));
        }
    }
}

fn handle_selected_world_heritage_site_change_event(
    mut event: EventReader<SelectedWorldHeritageSiteChangeEvent>,
    site_query: Query<(Entity, &WorldHeritageSiteComponent)>,
    mut selected_site: ResMut<SelectedWorldHeritageSite>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_query: Query<&Handle<StandardMaterial>, With<Handle<StandardMaterial>>>,
    mut transform_query: Query<&mut Transform>,
) {
    for event in event.iter() {
        let site = &event.0;

        if let Some(selected_site) = selected_site.site.as_ref() {
            for (entity, site_component) in site_query.iter() {
                if site_component.0 == *selected_site {
                    if let Ok(material_handle) = mesh_query.get(entity) {
                        if let Some(material) = materials.get_mut(material_handle) {
                            material.base_color = SITE_COLOR;
                        }
                    }
                    if let Ok(mut transform) = transform_query.get_mut(entity) {
                        transform.scale = Vec3::splat(1.0);
                    }
                    break;
                }
            }
        }

        for (entity, site_component) in site_query.iter() {
            if site_component.0 == *site {
                selected_site.site = Some(site.clone());
                if let Ok(material_handle) = mesh_query.get(entity) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = SITE_COLOR_SELECTED;
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
