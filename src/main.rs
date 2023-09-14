mod world;

use bevy::{prelude::*, window};
use bevy_rapier3d::prelude::*;
use character_controller_1::{
    character::{
        config::CharacterConfig,
        spawner::{spawn_character, CharacterSpawnSettings},
        Character, CharacterPlugin,
    },
    grounded::GroundedPlugin,
    player_movement_input::{PlayerMovementInput, PlayerMovementInputPlugin},
};
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CharacterPlugin,
            PlayerMovementInputPlugin,
            GroundedPlugin,
            WorldPlugin,
        ))
        .add_systems(Startup, spawn_test_character)
        .add_systems(Update, window::close_on_esc)
        .run();
}

fn _spawn_test_camera(mut commands: Commands) {
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

fn spawn_test_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_settings = CharacterSpawnSettings {
        spawn_position: Vec3::new(-1.0, 5.0, 0.0),
        ..default()
    };

    let character = Character {
        is_active: true,
        ..default()
    };

    let character_config = CharacterConfig {
        aerial_multiplier: 0.2,
        ..default()
    };

    let character_id = spawn_character(
        &mut commands,
        &mut meshes,
        &mut materials,
        character,
        character_config,
        &spawn_settings,
    );

    commands.entity(character_id).insert(PlayerMovementInput {
        hold_to_run: true,
        ..default()
    });
}
