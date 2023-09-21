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
    player_movement_input::PlayerMovementInputPlugin,
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
        .add_systems(Startup, (_spawn_test_camera, spawn_test_character))
        .add_systems(Update, (window::close_on_esc, test_input))
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
    let spawn_settings = CharacterSpawnSettings { ..default() };

    let character = Character {
        is_active: true,
        ..default()
    };

    let character_config = CharacterConfig { ..default() };

    CharacterSpawner::new(spawn_settings)
        .spawn_core(&mut commands, character, character_config)
        .add_body(&mut commands, &mut meshes, &mut materials)
        // .add_jumping(&mut commands)
        // .add_camera(&mut commands, build_third_person_camera(7.0))
        // .add_root_component(
        //     &mut commands,
        //     PlayerMovementInput {
        //         hold_to_run: true,
        //         ..default()
        //     },
        // )
        // .add_root_component(&mut commands, CharacterCrouch::new())
        ;
}

fn test_input(mut characters: Query<&mut Character>, input: Res<Input<KeyCode>>) {
    for mut character in characters.iter_mut() {
        let direction = walk_direction_from_input(&input);

        character.movement_input = direction;
    }
}

fn walk_direction_from_input(input: &Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        direction.z -= 1.0;
    }

    if input.pressed(KeyCode::S) {
        direction.z += 1.0;
    }

    if input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    direction.normalize_or_zero()
}
