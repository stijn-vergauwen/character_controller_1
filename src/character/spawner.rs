use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::grounded::{CheckMethod, Grounded};

use super::{config::CharacterConfig, jump::CharacterJump, Character, CharacterHead};

/*
    Entity hierarchy:
    - Character root (character components and rb, on Y: 0)
        - Character body (capsule with collider)
        - Character head (cube with collider)
            - First person camera
*/

pub struct CharacterSpawnSettings {
    pub color: Color,

    /// The percentage of the characters height that the head should take up
    pub head_percentage_of_height: f32,

    /// The value that the `Name` component of the character root will have
    pub character_name: String,

    /// The total size of the character, including the head
    ///
    /// NOTE: height needs to be large enough that `straight_height()` returns a value >= 0
    pub size: Vec2,

    pub spawn_position: Vec3,

    pub grounded_height_offset: f32,
    pub grounded_check_method: CheckMethod,
    pub draw_grounded_gizmos: bool,
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
            spawn_position: Vec3::ZERO,
            color: Color::CYAN,
            size: Vec2::new(0.8, 2.0),
            head_percentage_of_height: 20.0,
            character_name: String::from("Default character"),
            grounded_height_offset: 0.29,
            grounded_check_method: CheckMethod::Sphere { radius: 0.3 },
            draw_grounded_gizmos: false,
        }
    }
}

pub struct CharacterSpawner {
    spawn_settings: CharacterSpawnSettings,
    root_id: Option<Entity>,
    head_id: Option<Entity>,
}

impl CharacterSpawner {
    pub fn new(spawn_settings: CharacterSpawnSettings) -> Self {
        CharacterSpawner {
            spawn_settings,
            root_id: None,
            head_id: None,
        }
    }

    /// Spawns the core character components.
    ///
    /// Sets the `root_id` of this spawner.
    pub fn spawn_core(
        &mut self,
        commands: &mut Commands,
        character: Character,
        character_config: CharacterConfig,
    ) -> &mut Self {
        let id = commands
            .spawn((
                self.build_name_component(String::from("root")),
                build_rigid_body(character_config.drag_factor),
                TransformBundle::from_transform(Transform::from_translation(
                    self.spawn_settings.spawn_position,
                )),
                VisibilityBundle::default(),
                character,
                character_config,
            ))
            .id();

        self.root_id = Some(id);
        self
    }

    /// Spawns the body and head meshes and colliders of the character.
    ///
    /// Requires the `root_id` to be set, do this with the `spawn_base` function.
    pub fn add_body(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> &mut Self {
        if let Some(root_id) = self.root_id {
            let character_material = build_character_material(materials, &self.spawn_settings);

            commands.entity(root_id).with_children(|root| {
                // Body
                root.spawn((
                    self.build_name_component(String::from("body")),
                    build_character_body(meshes, character_material.clone(), &self.spawn_settings),
                ));

                // Head
                let id = root
                    .spawn((
                        self.build_name_component(String::from("head")),
                        build_character_head(
                            meshes,
                            character_material.clone(),
                            &self.spawn_settings,
                        ),
                    ))
                    .id();
                self.head_id = Some(id);
            });
        }
        self
    }

    pub fn add_jumping(&mut self, commands: &mut Commands) -> &mut Self {
        if let Some(root_id) = self.root_id {
            commands.entity(root_id).insert((
                Grounded::new(
                    self.spawn_settings.grounded_height_offset,
                    self.spawn_settings.grounded_check_method,
                    self.spawn_settings.draw_grounded_gizmos,
                ),
                CharacterJump::new(),
            ));
        }
        self
    }

    pub fn add_first_person_camera(&mut self, commands: &mut Commands) -> &mut Self {
        if let Some(head_id) = self.head_id {
            commands.entity(head_id).with_children(|head| {
                head.spawn((
                    self.build_name_component(String::from("first person camera")),
                    Camera3dBundle::default(),
                ));
            });
        }
        self
    }

    /// Adds the given component to the character root entity.
    pub fn add_root_component<T>(&mut self, commands: &mut Commands, component: T) -> &mut Self
    where
        T: Component,
    {
        if let Some(root_id) = self.root_id {
            commands.entity(root_id).insert(component);
        }
        self
    }

    fn build_name_component(&self, suffix: String) -> Name {
        Name::from(format!(
            "{} {}",
            self.spawn_settings.character_name.clone(),
            suffix
        ))
    }
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
