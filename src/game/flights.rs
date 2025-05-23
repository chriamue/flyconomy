use std::collections::HashMap;

use bevy::prelude::{in_state, App, Color, Gizmos, IntoSystemConfigs, Plugin, Res, Update};

use crate::game::{earth3d, projection::wgs84_to_xyz};
use crate::model::{Aerodrome, FlightState};
use crate::ui::views::UiView;

use super::{GameResource, GameState};

pub struct FlightsPlugin;

impl Plugin for FlightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (draw_flight_paths_system,).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (draw_flight_paths_analytics_system,)
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(UiView::Analytics)),
        );
    }
}

pub fn draw_flight_paths_system(game_resource: Res<GameResource>, mut gizmos: Gizmos) {
    let flight_query = game_resource.simulation.environment.flights.iter();

    for flight in flight_query {
        if let FlightState::EnRoute { .. } = flight.state {
            if let Some(destination) = flight.current_destination() {
                let origin = flight.current_origin();
                let start_gps = (origin.lat, origin.lon);
                let end_gps = (destination.lat, destination.lon);
                // Intermediate points in GPS
                let mut intermediate_gps_points = Vec::new();
                let num_intermediate = 32;
                for i in 1..=num_intermediate {
                    let t = i as f32 / (num_intermediate + 1) as f32; // Divide by total segments
                    let intermediate_lat = start_gps.0 + (end_gps.0 - start_gps.0) * t as f64;
                    let intermediate_lon = start_gps.1 + (end_gps.1 - start_gps.1) * t as f64;
                    intermediate_gps_points.push((intermediate_lat, intermediate_lon));
                }

                // Convert points to XYZ
                let mut points = vec![
                    wgs84_to_xyz(start_gps.0, start_gps.1, 0.0) * earth3d::SCALE_FACTOR as f32,
                ];
                for gps in intermediate_gps_points {
                    points.push(wgs84_to_xyz(gps.0, gps.1, 0.0) * earth3d::SCALE_FACTOR as f32);
                }
                points.push(wgs84_to_xyz(end_gps.0, end_gps.1, 0.0) * earth3d::SCALE_FACTOR as f32);

                gizmos.linestrip(points, Color::RED);
            }
        }
    }
}

pub fn draw_flight_paths_analytics_system(game_resource: Res<GameResource>, mut gizmos: Gizmos) {
    let flight_query = game_resource.simulation.environment.flights.iter();
    let mut route_counts: HashMap<(u64, u64), (Aerodrome, Aerodrome, usize)> = HashMap::new();
    let mut max_count = 0;

    for flight in flight_query.as_ref() {
        let mut aerodromes = vec![flight.origin_aerodrome.clone()];
        aerodromes.extend_from_slice(&flight.stopovers);
        aerodromes.push(flight.origin_aerodrome.clone());

        for aerodromes in aerodromes.windows(2) {
            let aerodrome1 = &aerodromes[0];
            let aerodrome2 = &aerodromes[1];

            let (smaller_id, larger_id, origin, destination) = if aerodrome1.id < aerodrome2.id {
                (
                    aerodrome1.id,
                    aerodrome2.id,
                    aerodrome1.clone(),
                    aerodrome2.clone(),
                )
            } else {
                (
                    aerodrome2.id,
                    aerodrome1.id,
                    aerodrome2.clone(),
                    aerodrome1.clone(),
                )
            };

            let (_, _, count) =
                route_counts
                    .entry((smaller_id, larger_id))
                    .or_insert((origin, destination, 0));

            *count += 1;
            max_count = max_count.max(*count);
        }
    }

    for ((_, _), (origin, destination, count)) in route_counts {
        let intensity = count as f32 / max_count as f32;
        let color = Color::rgb(intensity, 0.0, 1.0 - intensity);

        let start_gps = (origin.lat, origin.lon);
        let end_gps = (destination.lat, destination.lon);

        let mut intermediate_gps_points = Vec::new();
        let num_intermediate = 32;

        for i in 1..=num_intermediate {
            let t = i as f32 / (num_intermediate + 1) as f32; // Divide by total segments
            let intermediate_lat = start_gps.0 + (end_gps.0 - start_gps.0) * t as f64;
            let intermediate_lon = start_gps.1 + (end_gps.1 - start_gps.1) * t as f64;
            intermediate_gps_points.push((intermediate_lat, intermediate_lon));
        }

        let mut points =
            vec![wgs84_to_xyz(start_gps.0, start_gps.1, 0.0) * earth3d::SCALE_FACTOR as f32];

        for gps in intermediate_gps_points {
            points.push(wgs84_to_xyz(gps.0, gps.1, 0.0) * earth3d::SCALE_FACTOR as f32);
        }

        points.push(wgs84_to_xyz(end_gps.0, end_gps.1, 0.0) * earth3d::SCALE_FACTOR as f32);

        gizmos.linestrip(points, color);
    }
}
