use bevy::prelude::{App, Assets, Color, Commands, Component, Entity, Query, ResMut, With};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use bevy_polyline::PolylinePlugin;

use crate::game::{earth3d, projection::wgs84_to_xyz};

use super::GameResource;

pub fn add_flight_systems_to_app(app: &mut App) {
    app.add_plugin(PolylinePlugin)
        .add_system(draw_flight_paths_system);
}

pub fn draw_flight_paths_system(
    mut commands: Commands,
    game_resource: ResMut<GameResource>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    line_query: Query<Entity, With<FlightLine>>,
) {
    for line in line_query.iter() {
        commands.entity(line).despawn();
    }

    let flight_query = game_resource.simulation.environment.flights.iter();

    for flight in flight_query {
        let origin = &flight.origin_aerodrome;
        let destination = &flight.destination_aerodrome;
        let start_point = wgs84_to_xyz(origin.lat, origin.lon, 0.0) * earth3d::SCALE_FACTOR as f32;
        let end_point =
            wgs84_to_xyz(destination.lat, destination.lon, 0.0) * earth3d::SCALE_FACTOR as f32;
        commands.spawn(PolylineBundle {
            polyline: polylines.add(Polyline {
                vertices: vec![start_point, end_point],
            }),
            material: polyline_materials.add(PolylineMaterial {
                width: 2.0,
                color: Color::RED,
                perspective: false,
                depth_bias: -0.0002,
            }),
            ..Default::default()
        });
    }
}

#[derive(Default, Component)]
pub struct FlightLine;
