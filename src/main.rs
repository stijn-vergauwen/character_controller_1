mod world;

use bevy::{prelude::*, window};
use bevy_rapier3d::prelude::*;
use character_controller_1::{
    character::{
        camera::build_third_person_camera,
        config::CharacterConfig,
        crouch::CharacterCrouch,
        spawner::{CharacterSpawnSettings, CharacterSpawner},
        Character, CharacterPlugin,
    },
    grounded::{GroundedPlugin, CheckMethod},
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
        spawn_position: Vec3::new(0.0, 2.0, 0.0),
        grounded_check_method: CheckMethod::Ray { distance: 0.3 },
        grounded_height_offset: 0.1,
        draw_grounded_gizmos: true,
        ..default()
    };

    let character = Character {
        is_active: true,
        ..default()
    };

    let character_config = CharacterConfig { ..default() };

    CharacterSpawner::new(spawn_settings)
        .spawn_core(&mut commands, character, character_config)
        .add_body(&mut commands, &mut meshes, &mut materials)
        .add_jumping(&mut commands)
        .add_camera(&mut commands, build_third_person_camera(7.0))
        .add_root_component(
            &mut commands,
            PlayerMovementInput {
                hold_to_run: true,
                ..default()
            },
        )
        .add_root_component(&mut commands, CharacterCrouch::new());
}
