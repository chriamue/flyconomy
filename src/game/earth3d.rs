// source: https://github.com/nicopap/bevy_mod_paramap/blob/main/examples/earth3d.rs

use std::f32::consts::TAU;

use bevy::{prelude::*, render::render_resource::TextureFormat};

pub const EARTH_RADIUS: f64 = 6_378_137.0;
pub const SIMULATION_EARTH_RADIUS: f64 = 1.0;
pub const SCALE_FACTOR: f64 = SIMULATION_EARTH_RADIUS / EARTH_RADIUS;

const NORMAL_MAP: &str = "earth/normal_map.jpg";
const HEIGHT_MAP: &str = "earth/elevation_surface.jpg";
const ROUGH_MAP: &str = "earth/metallic_roughness.png";
const ALBEDO_MAP: &str = "earth/base_color.jpg";
const EMI_MAP: &str = "earth/emissive.jpg";
const SPIN: f32 = 0.0;

pub struct Earth3dPlugin;

impl Plugin for Earth3dPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.01,
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Normal(None));
        app.add_systems(Startup, (setup,))
            .add_systems(Update, (update_normal, spin));
        app.register_type::<Spin>();
    }
}

#[derive(Component, PartialEq, Eq)]
struct Earth;

#[derive(Component, PartialEq, Reflect)]
struct Spin(f32);

fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>) {
    for (mut transform, spin) in query.iter_mut() {
        transform.rotate_y(spin.0 * time.delta_seconds());
    }
}

/// Store handle of the earth normal to later modify its format
/// in [`update_normal`].
#[derive(Resource)]
struct Normal(Option<Handle<Image>>);

/// Work around the fact that the default bevy image loader sets the
/// normal's format to something incompatible with normal shaders.
/// The format must be one of the `TextureFormat` ending in `*Unorm`.
///
/// In this function, we wait until the image is loaded, immediately
/// change its format and never run the core logic afterward.
///
/// Without proper format, it looks like the light source moves as the
/// earth move, and there is major glitchy artifacts on the poles.
fn update_normal(
    mut already_ran: Local<bool>,
    mut images: ResMut<Assets<Image>>,
    normal: Res<Normal>,
) {
    if *already_ran {
        return;
    }
    if let Some(normal) = normal.0.as_ref() {
        if let Some(image) = images.get_mut(normal) {
            image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
            *already_ran = true;
        }
    }
}

/// setup earth and point light.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut normal: ResMut<Normal>,
    assets: Res<AssetServer>,
) {
    use bevy::math::EulerRot::XYZ;
    let normal_handle = assets.load(NORMAL_MAP);
    normal.0 = Some(normal_handle.clone());
    let mut sphere: Mesh = shape::UVSphere::default().into();
    sphere.generate_tangents().unwrap();

    let earth_material = {
        materials.add(StandardMaterial {
            perceptual_roughness: 0.75,
            base_color_texture: Some(assets.load(ALBEDO_MAP)),
            normal_map_texture: Some(normal_handle),
            emissive: Color::rgb_u8(30, 30, 30),
            emissive_texture: Some(assets.load(EMI_MAP)),
            metallic_roughness_texture: Some(assets.load(ROUGH_MAP)),
            depth_map: Some(assets.load(HEIGHT_MAP)),
            parallax_depth_scale: 0.0025,
            ..Default::default()
        })
    };

    commands
        .spawn(MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_euler(XYZ, -TAU / 4.0, 0.0, TAU / 2.0)),
            mesh: meshes.add(sphere),
            material: earth_material,
            ..default()
        })
        .insert((Earth, Spin(SPIN), Name::new("Earth")));
}
