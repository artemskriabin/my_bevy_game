use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, PointLight, StandardMaterial};
use bevy::prelude::{
    shape, Camera, Camera3d, Camera3dBundle, Commands, Component, GlobalTransform, Image,
    ImagePlugin, KeyCode, Mesh, PluginGroup, PointLightBundle, Query, Res, ResMut, Resource,
    Transform, With, Without,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::time::Time;
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(HelloPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpeedCoefficient(1.0))
            .add_startup_system(setup_env)
            .add_system(fly_forward)
            .add_system(control_camera)
            //.add_system(fly_away)
            .add_system(print_ball_altitude)
            .add_system(boom);
    }
}

fn setup_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    //floor
    commands.spawn((
        Collider::cuboid(100.0, 0.0, 100.0),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(100.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(images.add(uv_debug_texture())),
                ..default()
            }),
            ..default()
        },
    ));

    /* Create the bouncing ball. */
    // target
    commands
        .spawn((
            RigidBody::Dynamic,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(images.add(uv_debug_texture())),
                    ..default()
                }),
                transform: Transform::from_xyz(1.0, 13.5, 11.0),
                ..default()
            },
            Target,
        ))
        .insert(Collider::ball(1.0))
        .insert(Restitution {
            coefficient: 0.9,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, -1.0),
            angvel: Vec3::new(-1.0, 0.0, 0.0),
        })
        .insert(Damping {
            linear_damping: 0.01,
            angular_damping: 0.01,
        });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 12.5, 25.0)
            .looking_at(Vec3::new(0.0, 9.0, 0.0), Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3500.0,
            shadows_enabled: true,
            range: 100.0,
            radius: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 15.0, 4.0),
        ..default()
    });
}

fn control_camera(
    time: Res<Time>,
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut camera_transform, _) = camera.single_mut();

    let rotation = if input.pressed(KeyCode::Left) {
        time.delta_seconds()
    } else if input.pressed(KeyCode::Right) {
        -time.delta_seconds()
    } else {
        0.0
    };

    let tilt = if input.pressed(KeyCode::Up) {
        -time.delta_seconds()
    } else if input.pressed(KeyCode::Down) {
        time.delta_seconds()
    } else {
        0.0
    };

    camera_transform.rotate_local_x(tilt);
    camera_transform.rotate_y(rotation);
}

#[derive(Resource)]
struct SpeedCoefficient(f32);

fn fly_forward(
    time: Res<Time>,
    mut coefficient: ResMut<SpeedCoefficient>,
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut camera_transform, _) = camera.single_mut();

    let delta = time.delta_seconds() * coefficient.0;
    let forward = camera_transform.forward();

    camera_transform.translation.z += forward.z * delta;
    camera_transform.translation.x += forward.x * delta;
    camera_transform.translation.y += forward.y * delta;

    let multiplier: f32 = if input.pressed(KeyCode::X) {
        1.1
    } else if input.pressed(KeyCode::Z) {
        0.9
    } else {
        1.0
    };

    coefficient.as_mut().0 *= multiplier;
}

#[derive(Component)]
struct Target;

#[allow(dead_code)]
fn fly_away(mut query: Query<&mut Transform, With<Target>>, time: Res<Time>) {
    for mut transform in &mut query {
        let delta = time.delta_seconds();
        transform.translation.z -= delta;
        transform.rotate_local_x(-delta);
    }
}

fn boom(
    query: Query<&mut Transform, With<Target>>,
    camera: Query<
        (&mut Camera, &mut Transform, &GlobalTransform),
        (With<Camera3d>, Without<Target>),
    >,
) {
    let (_, camera_transform, _) = camera.single();

    let cube = query.single();

    if camera_transform
        .translation
        .abs_diff_eq(cube.translation, 1.0)
    {
        panic!("boom")
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
