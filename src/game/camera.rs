use std::f32::consts::TAU;

use bevy::{app::PluginGroupBuilder, core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub fn add_camera_systems_to_app(app: &mut App) {
    app.add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup_camera)
        .add_system(keyboard_controls);
}

pub fn setup_camera(mut commands: Commands) {
    let mut camera = commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: !cfg!(target_arch = "wasm32"),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Left,
            radius: Some(3.0),
            alpha: Some(TAU / 4.0),
            beta: Some(TAU / 8.0),
            zoom_sensitivity: 0.2,
            pan_sensitivity: 0.2,
            orbit_sensitivity: 0.2,
            ..default()
        },
        RaycastPickCamera::default(),
    ));
    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(BloomSettings {
        intensity: 0.1,
        ..default()
    });
}

fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
        if key_input.pressed(KeyCode::Right) {
            pan_orbit.target_alpha += 50f32.to_radians() * time.delta_seconds();
        }
        if key_input.pressed(KeyCode::Left) {
            pan_orbit.target_alpha -= 50f32.to_radians() * time.delta_seconds();
        }
        if key_input.pressed(KeyCode::Up) {
            pan_orbit.target_beta += 50f32.to_radians() * time.delta_seconds();
        }
        if key_input.pressed(KeyCode::Down) {
            pan_orbit.target_beta -= 50f32.to_radians() * time.delta_seconds();
        }

        // Zoom with Z and X
        if key_input.pressed(KeyCode::Z) {
            pan_orbit.radius = pan_orbit
                .radius
                .map(|radius| radius - 5.0 * time.delta_seconds());
        }
        if key_input.pressed(KeyCode::X) {
            pan_orbit.radius = pan_orbit
                .radius
                .map(|radius| radius + 5.0 * time.delta_seconds());
        }
        pan_orbit.force_update = true;
    }
}

pub struct CameraPlugins;

impl PluginGroup for CameraPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PanOrbitCameraPlugin)
    }
}
