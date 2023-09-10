use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player_movement_input::PlayerMovementInput;

use super::Character;

const WALKING_STRENGTH: f32 = 8.0;
const RUNNING_STRENGTH: f32 = 13.0;
const ROTATION_STRENGTH: f32 = 0.0007;
const JUMP_STRENGTH: f32 = 3.0;

const CHARACTER_COLOR: Color = Color::ORANGE;

const HEAD_SIZE: f32 = 0.5;
const HEAD_HEIGHT_OFFSET: f32 = 1.0;

pub struct CharacterSpawnSettings {
    spawn_position: Vec3,
    color: Color,

    /// The total size of the character, including the head
    ///
    /// NOTE: height needs to be large enough that `straight_height()` returns a value >= 0
    size: Vec2,

    /// The percentage of the characters height that the head should take up
    head_percentage_of_height: f32,
}

impl CharacterSpawnSettings {
    fn head_height(&self) -> f32 {
        self.size.y * (self.head_percentage_of_height / 100.0)
    }

    fn body_height(&self) -> f32 {
        self.size.y - self.head_height()
    }

    fn half_body_height(&self) -> f32 {
        self.body_height() / 2.0
    }

    fn half_body_width(&self) -> f32 {
        self.size.x / 2.0
    }

    fn radius(&self) -> f32 {
        self.half_body_width()
    }

    fn straight_height(&self) -> f32 {
        self.body_height() - self.size.x
    }

    fn head_height_offset(&self) -> f32 {
        self.half_body_height() + self.head_height() / 2.0
    }
}

impl Default for CharacterSpawnSettings {
    fn default() -> Self {
        Self {
            spawn_position: Vec3::Y,
            color: Color::CYAN,
            size: Vec2::new(0.8, 2.0),
            head_percentage_of_height: 15.0,
        }
    }
}

pub fn spawn_default_character_with_user_input(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let spawn_settings = CharacterSpawnSettings::default();

    commands
        .spawn((
            Name::from("Default character body"),
            build_character_body(meshes, materials, &spawn_settings),
            build_rigid_body(1.0),
            Character::new(
                WALKING_STRENGTH,
                RUNNING_STRENGTH,
                JUMP_STRENGTH,
                ROTATION_STRENGTH,
            ),
            PlayerMovementInput::default(),
        ))
        .with_children(|body| {
            // Head
            body.spawn((
                Name::from("Default character head"),
                PbrBundle {
                    mesh: meshes.add(shape::Cube::new(HEAD_SIZE).into()),
                    material: materials.add(StandardMaterial {
                        base_color: CHARACTER_COLOR,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, HEAD_HEIGHT_OFFSET, 0.0),
                    ..default()
                },
            ))
            .with_children(|head| {
                spawn_character_camera(head);
            });
        });
}

fn spawn_character_camera(parent: &mut ChildBuilder) {
    parent.spawn((
        Name::from("Character first-person camera"),
        Camera3dBundle { ..default() },
    ));
}

/// Returns a dynamic rigidbody with relevant components for characters
///
/// * Linear damping simulates the strength of air resistance
fn build_rigid_body(
    linear_damping: f32,
) -> (
    RigidBody,
    LockedAxes,
    ExternalForce,
    ExternalImpulse,
    Damping,
) {
    (
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        ExternalForce::default(),
        ExternalImpulse::default(),
        Damping {
            linear_damping,
            angular_damping: 0.0,
        },
    )
}

/// Returns a capsule with collider that has the given size and color
fn build_character_body(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_settings: &CharacterSpawnSettings,
) -> (PbrBundle, Collider) {
    (
        PbrBundle {
            mesh: meshes.add(
                shape::Capsule {
                    depth: spawn_settings.straight_height(),
                    radius: spawn_settings.radius(),
                    ..default()
                }
                .into(),
            ),
            material: materials.add(StandardMaterial {
                base_color: spawn_settings.color,
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_translation(spawn_settings.spawn_position),
            ..default()
        },
        Collider::capsule_y(
            spawn_settings.straight_height() / 2.0,
            spawn_settings.radius(),
        ),
    )
}