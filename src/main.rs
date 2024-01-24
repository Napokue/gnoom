mod camera_system;
mod gnoem;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_rapier3d::prelude::*;
use gnoem::GnoemPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, camera_system::build_plugins(), GnoemPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a static ground plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Collider::cuboid(50.0, 0.1, 50.0));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera_system
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 5.,
            scaling_mode: ScalingMode::FixedVertical(2.),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
