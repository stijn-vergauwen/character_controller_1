mod world;

use bevy::{prelude::*, window};
use bevy_rapier3d::prelude::*;
use character_controller_1::{
    character::{
        config::CharacterConfig,
        spawner::{CharacterSpawnSettings, CharacterSpawner},
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
        draw_grounded_gizmos: true,
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

    let linear_damping = character_config.drag_factor;

    CharacterSpawner::new(spawn_settings)
        .spawn_core(&mut commands, character, character_config)
        .add_body(&mut commands, &mut meshes, &mut materials)
        .add_rigid_body(&mut commands, linear_damping)
        .add_jumping(&mut commands)
        .add_first_person_camera(&mut commands)
        .add_root_component(
            &mut commands,
            PlayerMovementInput {
                hold_to_run: true,
                ..default()
            },
        );
}
