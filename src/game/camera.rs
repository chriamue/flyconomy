use std::f32::consts::TAU;

use bevy::{app::PluginGroupBuilder, core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, (setup_camera,))
            .add_systems(Update, (keyboard_controls,));
    }
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
            zoom_sensitivity: 0.15,
            pan_sensitivity: 0.2,
            orbit_sensitivity: 0.2,
            ..default()
        },
    ));
    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(BloomSettings {
        intensity: 0.1,
        ..default()
    });
}

fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
        if key_input.pressed(KeyCode::ArrowRight) {
            pan_orbit.target_alpha += 50f32.to_radians() * time.delta_seconds();
        }
        if key_input.pressed(KeyCode::ArrowLeft) {
            pan_orbit.target_alpha -= 50f32.to_radians() * time.delta_seconds();
        }
        if key_input.pressed(KeyCode::ArrowUp) {
            pan_orbit.target_beta += 50f32.to_radians() * time.delta_seconds();
        }
        if key_input.pressed(KeyCode::ArrowDown) {
            pan_orbit.target_beta -= 50f32.to_radians() * time.delta_seconds();
        }

        // Zoom with Z and X
        if key_input.pressed(KeyCode::KeyZ) {
            pan_orbit.radius = pan_orbit
                .radius
                .map(|radius| radius - 1.0 * time.delta_seconds());
        }
        if key_input.pressed(KeyCode::KeyX) {
            pan_orbit.radius = pan_orbit
                .radius
                .map(|radius| radius + 1.0 * time.delta_seconds());
        }

        if pan_orbit.radius < Some(1.2) {
            pan_orbit.radius = Some(1.2);
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
