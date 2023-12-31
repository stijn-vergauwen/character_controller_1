mod world;

use bevy::{prelude::*, window};
use bevy_rapier3d::prelude::*;
#[allow(unused_imports)]
use character_controller_1::character::camera::{
    build_first_person_camera, build_third_person_camera,
};
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
            // RapierDebugRenderPlugin::default(),
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
    let spawn_settings = CharacterSpawnSettings::default();
    let character = Character::default();
    let character_config = CharacterConfig::default();

    // This component is optional, you can also use your own input handling.
    let movement_input = PlayerMovementInput::default();

    CharacterSpawner::new(spawn_settings)
        .spawn_core(&mut commands, character, character_config)
        .add_body(&mut commands, &mut meshes, &mut materials)
        .add_jumping(&mut commands)
        // .add_camera(&mut commands, build_first_person_camera())
        .add_camera(&mut commands, build_third_person_camera(7.0))
        .add_root_component(&mut commands, movement_input);
}
