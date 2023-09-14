use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::grounded::{CastMethod, Grounded};

use super::{config::CharacterConfig, jump::CharacterJump, Character, CharacterHead};

pub struct CharacterSpawnSettings {
    pub color: Color,

    /// The percentage of the characters height that the head should take up
    pub head_percentage_of_height: f32,

    /// The value that the `Name` component of the character root will have
    pub root_name: String,

    /// The total size of the character, including the head
    ///
    /// NOTE: height needs to be large enough that `straight_height()` returns a value >= 0
    pub size: Vec2,

    pub spawn_position: Vec3,

    pub grounded_check_distance: f32,
    pub grounded_check_offset: f32,
    pub grounded_check_method: CastMethod,
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

/*
    Entity hierarchy:
    - Character root (character components and rb, on Y: 0)
        - Character body (capsule with collider)
        - Character head (cube with collider)
            - First person camera
*/

impl Default for CharacterSpawnSettings {
    fn default() -> Self {
        Self {
            spawn_position: Vec3::ZERO,
            color: Color::CYAN,
            size: Vec2::new(0.8, 2.0),
            head_percentage_of_height: 20.0,
            root_name: String::from("Default character root"),
            grounded_check_distance: 0.1,
            grounded_check_offset: 0.0,
            grounded_check_method: CastMethod::Ray,
        }
    }
}

// TODO: split up into multiple functions, add one's for: grounded + jump, camera

/// Spawns a character complete with a body, head, rigidbody, colliders, and first-person camera.
///
/// Returns the character root entity
pub fn spawn_character(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    character: Character,
    character_config: CharacterConfig,
    spawn_settings: &CharacterSpawnSettings,
) -> Entity {
    let character_material = build_character_material(materials, spawn_settings);

    commands
        .spawn((
            Name::from(spawn_settings.root_name.clone()),
            build_rigid_body(character_config.drag_factor),
            character,
            character_config,
            TransformBundle::from_transform(Transform::from_translation(
                spawn_settings.spawn_position,
            )),
            VisibilityBundle::default(),
            Grounded::new(
                spawn_settings.grounded_check_distance,
                spawn_settings.grounded_check_offset,
                spawn_settings.grounded_check_method,
            ),
            CharacterJump::new(),
        ))
        .with_children(|root| {
            // Body
            root.spawn((
                Name::from("Character body"),
                build_character_body(meshes, character_material.clone(), spawn_settings),
            ));

            // Head
            root.spawn((
                Name::from("Character head"),
                build_character_head(meshes, character_material.clone(), spawn_settings),
            ))
            .with_children(|head| {
                head.spawn((Name::from("Character camera"), Camera3dBundle::default()));
            });
        })
        .id()
}

/// Returns a dynamic rigidbody with relevant components for characters
///
/// * Linear damping simulates the strength of air resistance
fn build_rigid_body(
    linear_damping: f32,
) -> (
    RigidBody,
    Velocity,
    LockedAxes,
    ExternalForce,
    ExternalImpulse,
    Damping,
) {
    (
        RigidBody::Dynamic,
        Velocity::default(),
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
    material_handle: Handle<StandardMaterial>,
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
            material: material_handle,
            transform: Transform::from_xyz(0.0, spawn_settings.half_body_height(), 0.0),
            ..default()
        },
        Collider::capsule_y(
            spawn_settings.straight_height() / 2.0,
            spawn_settings.radius(),
        ),
    )
}

fn build_character_head(
    meshes: &mut ResMut<Assets<Mesh>>,
    material_handle: Handle<StandardMaterial>,
    spawn_settings: &CharacterSpawnSettings,
) -> (PbrBundle, Collider, CharacterHead) {
    let head_size = spawn_settings.head_height();
    (
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(head_size).into()),
            material: material_handle,
            transform: Transform::from_xyz(
                0.0,
                spawn_settings.half_body_height() + spawn_settings.head_height_offset(),
                0.0,
            ),
            ..default()
        },
        Collider::cuboid(head_size / 2.0, head_size / 2.0, head_size / 2.0),
        CharacterHead,
    )
}

fn build_character_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_settings: &CharacterSpawnSettings,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: spawn_settings.color,
        perceptual_roughness: 1.0,
        ..default()
    })
}
