use bevy::prelude::{
    App, AssetServer, BuildChildren, ChildBuilder, Commands, Component, DespawnRecursiveExt,
    Entity, GlobalTransform, Plugin, Quat, Query, Res, Transform, Update, Vec3, Visibility,
};
use bevy::render::view::InheritedVisibility;
use bevy::scene::SceneBundle;
use bevy_obj::ObjPlugin;
use std::collections::HashMap;

use super::{projection::wgs84_to_xyz, GameResource};
use crate::{game::earth3d, model::FlightState};

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ObjPlugin).add_systems(Update, plane_system);
    }
}

#[derive(Component)]
pub struct FlightVisual {
    pub flight_id: u64,
}

pub fn plane_system(
    mut commands: Commands,
    mut query: Query<(Entity, &FlightVisual, &mut Transform)>,
    asset_server: Res<AssetServer>,
    game_resource: Res<GameResource>,
) {
    let current_time = game_resource.simulation.environment.timestamp;

    // Track which flight IDs are already represented by visuals
    let mut visualized_flights: HashMap<u64, Entity> = HashMap::new();
    for (entity, flight_visual, _) in query.iter_mut() {
        visualized_flights.insert(flight_visual.flight_id, entity);
    }

    // Loop through flights
    for flight in game_resource.simulation.environment.flights.iter() {
        let should_display = match flight.state {
            FlightState::EnRoute { .. } => true,
            _ => false,
        };

        if should_display {
            if let (Some((current_lat, current_lon)), Some(destination)) = (
                flight.estimate_current_position(current_time),
                flight.current_destination(),
            ) {
                let position =
                    wgs84_to_xyz(current_lat, current_lon, 10_000.0) * earth3d::SCALE_FACTOR as f32;
                // Calculate the look-at rotation
                let destination = wgs84_to_xyz(destination.lat, destination.lon, 1_000.0)
                    * earth3d::SCALE_FACTOR as f32;

                if let Some(entity) = visualized_flights.get(&flight.flight_id) {
                    // Update the position of an existing plane

                    if let Ok(mut transform) = query.get_component_mut::<Transform>(*entity) {
                        transform.translation = position;
                        transform.look_at(destination, Vec3::Y);
                    }
                } else {
                    let mut transform = Transform {
                        translation: position,
                        ..Default::default()
                    };
                    transform.look_at(destination, Vec3::Y);
                    commands
                        .spawn((
                            transform,
                            GlobalTransform::default(),
                            Visibility::Inherited,
                            InheritedVisibility::default(),
                            FlightVisual {
                                flight_id: flight.flight_id,
                            },
                        ))
                        .with_children(|child_builder| match flight.airplane.plane_type.id {
                            0 => small_plane_model(child_builder, &asset_server),
                            1 => medium_plane_model(child_builder, &asset_server),
                            2 => big_plane_model(child_builder, &asset_server),
                            _ => small_plane_model(child_builder, &asset_server),
                        });
                }
            }
        } else if let Some(entity) = visualized_flights.get(&flight.flight_id) {
            commands.entity(*entity).remove::<FlightVisual>();
            commands.entity(*entity).despawn_recursive();
        }
    }
}

fn small_plane_model(child_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let scale = Vec3::new(1.0, 1.0, 1.0) * 12_000.0 * earth3d::SCALE_FACTOR as f32;

    child_builder.spawn(SceneBundle {
        scene: asset_server.load("airplanes/business_jet.glb#Scene0"),
        transform: Transform {
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            scale,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn medium_plane_model(child_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let scale = Vec3::new(1.0, 1.0, 1.0) * 12_000.0 * earth3d::SCALE_FACTOR as f32;

    child_builder.spawn(SceneBundle {
        scene: asset_server.load("airplanes/airbus.glb#Scene0"),
        transform: Transform {
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            scale,
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn big_plane_model(child_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let scale = Vec3::new(1.0, 1.0, 1.0) * 25.0 * 12_000.0 * earth3d::SCALE_FACTOR as f32;

    child_builder.spawn(SceneBundle {
        scene: asset_server.load("airplanes/boeing.glb#Scene0"),
        transform: Transform {
            rotation: Quat::from_rotation_y(std::f32::consts::PI * -0.5),
            scale,
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
