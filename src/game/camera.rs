use bevy::{app::PluginGroupBuilder, core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub fn setup_camera(mut commands: Commands) {
    let mut camera = commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: !cfg!(target_arch = "wasm32"),
                ..default()
            },
            transform: Transform::from_xyz(3.9, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Left,
            ..default()
        },
    ));
    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(BloomSettings {
        intensity: 0.1,
        ..default()
    });
}

pub struct CameraPlugins;

impl PluginGroup for CameraPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PanOrbitCameraPlugin)
    }
}
