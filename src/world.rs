use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_static_objects, spawn_light));
    }
}

fn spawn_static_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Name::from("Ground plane"),
        Collider::cuboid(100.0, 0.1, 100.0),
        PbrBundle {
            mesh: meshes.add(shape::Box::new(200.0, 0.2, 200.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Cube
    commands.spawn((
        Name::from("Orange test cube"),
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(2.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 3.5, -10.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::from("Directional light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 20.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
    ));
}
