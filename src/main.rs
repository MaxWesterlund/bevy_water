//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    render::camera::ScalingMode,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("372f3a").unwrap()
        ))
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_material_time)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 10.0
            },
            ..default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 512.0, 
                min_height: 288.0
            },
            ..default()
        },
        ..default()
    });

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(256.0)),
        material: materials.add(CustomMaterial {
            light_color: Color::hex("6a9395").unwrap(),
            medium_light_color: Color::hex("5d7680").unwrap(),
            medium_dark_color: Color::hex("545e72").unwrap(),
            dark_color: Color::hex("464459").unwrap(),
            time: 0.0,
        }),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    light_color: Color,
    #[uniform(1)]
    medium_light_color: Color,
    #[uniform(2)]
    medium_dark_color: Color,
    #[uniform(3)]
    dark_color: Color,
    #[uniform(4)]
    time: f32,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_2d.wgsl".into()
    }
}

fn update_material_time(
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
    }
}
