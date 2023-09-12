use std::f32::consts::PI;

use bevy::{prelude::*, window};
use bevy_rapier3d::prelude::*;
use character_controller_1::{
    character::{
        spawner::{spawn_character, CharacterSpawnSettings},
        Character, CharacterPlugin,
    },
    player_movement_input::{PlayerMovementInput, PlayerMovementInputPlugin},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CharacterPlugin,
            PlayerMovementInputPlugin,
        ))
        .add_systems(
            Startup,
            (_spawn_camera, spawn_objects, spawn_test_character),
        )
        .add_systems(Update, window::close_on_esc)
        .run();
}

fn _spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::from("Scene camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(-6.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 20,
                ..default()
            },
            ..Default::default()
        },
    ));
}

fn spawn_objects(
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
            transform: Transform::from_xyz(1.0, 1.5, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    // Light
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

fn spawn_test_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_settings = CharacterSpawnSettings {
        spawn_position: Vec3::new(-1.0, 0.0, 0.0),
        drag_factor: 0.5,
        ..default()
    };

    let character_id = spawn_character(
        &mut commands,
        &mut meshes,
        &mut materials,
        Character::default(),
        &spawn_settings,
    );

    commands
        .entity(character_id)
        .insert(PlayerMovementInput::default());
}
