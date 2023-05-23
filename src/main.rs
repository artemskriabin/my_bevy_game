use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::input::Input;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{
    shape, Camera, Camera3d, Camera3dBundle, Color, Commands, GlobalTransform, KeyCode, Mesh,
    Query, Res, ResMut, Transform, With,
};
use bevy::time::Time;
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
        app.add_startup_system(setup_env).add_system(control_camera);
    }
}

fn setup_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    camera_transform.rotate_around(
        camera_global_transform.translation(),
        Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
    );
}
