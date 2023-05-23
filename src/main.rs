use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::input::Input;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{
    shape, Camera, Camera3d, Camera3dBundle, Color, Commands, GlobalTransform, KeyCode, Mesh,
    Query, Res, ResMut, Resource, Transform, With,
};
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FlyTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_startup_system(setup_env)
            .add_system(fly)
            .add_system(control_camera);
    }
}

fn setup_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn control_camera(
    time: Res<Time>,
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut camera_transform, camera_global_transform) = camera.single_mut();

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
    camera_transform.rotate_local_y(rotation);
}

#[derive(Resource)]
struct FlyTimer(Timer);

fn fly(
    time: Res<Time>,
    mut timer: ResMut<FlyTimer>,
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
) {
    let (mut camera, mut camera_transform, camera_global_transform) = camera.single_mut();

    let delta = time.delta_seconds();
    let forward = camera_transform.forward();
    camera_transform.translation.z += forward.z * delta;
    camera_transform.translation.x += forward.x * delta;
    camera_transform.translation.y += forward.y * delta;
}
