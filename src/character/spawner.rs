use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::grounded::{CheckMethod, Grounded};

use super::{
    config::CharacterConfig, jump::CharacterJump, Character, CharacterBody, CharacterHead,
};

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
    /// * Note: height needs to be large enough that `straight_height()` returns a value >= 0
    pub size: Vec2,

    /// The spawn position for the root of the character.
    ///
    /// * Note: the character root is positioned at ground height. So if your ground is at Y=0, setting `spawn_position` to Y=0 spawns the character on the ground.
    pub spawn_position: Vec3,

    pub grounded_height_offset: f32,
    pub grounded_check_method: CheckMethod,

    /// If gizmos should be drawn to show the shape or ray used for checking if the character is grounded.
    ///
    /// Useful when tuning the grounded behaviour
    pub draw_grounded_gizmos: bool,

    /// The amount of drag or air resistance this character will experience.
    ///
    /// This is the value that the rigidbody's `linear_damping` will be set to.
    pub drag: f32,

    /// The amount of friction the body collider will have.
    pub friction: f32,
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

    /// Returns the height that the straight section of the character's body will have.
    ///
    /// This straight section is the middle cylinder part of the capsule shape.
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
            size: Vec2::new(0.65, 2.0),
            head_percentage_of_height: 20.0,
            character_name: String::from("Default character"),
            grounded_height_offset: 0.15,
            grounded_check_method: CheckMethod::Sphere { radius: 0.2 },
            draw_grounded_gizmos: false,
            drag: 0.5,
            friction: 0.3,
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
                build_rigid_body(self.spawn_settings.drag),
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
    /// Requires the `root_id` to be set, do this with the `spawn_core` function.
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

    /// Spawns `Grounded` and `CharacterJump` components on the character root entity.
    ///
    /// Requires the `root_id` to be set, do this with the `spawn_core` function.
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

    /// Spawns a default `Camera3dBundle` component as child entity of the character head.
    ///
    /// Requires the `head_id` to be set, do this with the `add_body` function.
    ///
    /// You can use the `build_first_person_camera` or `build_third_person_camera` for making the camera component
    pub fn add_camera(&mut self, commands: &mut Commands, component: impl Bundle) -> &mut Self {
        if let Some(head_id) = self.head_id {
            commands.entity(head_id).with_children(|head| {
                head.spawn((self.build_name_component(String::from("camera")), component));
            });
        }
        self
    }

    /// Adds the given component to the character root entity.
    pub fn add_root_component(
        &mut self,
        commands: &mut Commands,
        component: impl Bundle,
    ) -> &mut Self {
        if let Some(root_id) = self.root_id {
            commands.entity(root_id).insert(component);
        }
        self
    }

    /// Returns the `root_id` of this character.
    pub fn id(&self) -> Option<Entity> {
        self.root_id
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
) -> (PbrBundle, Collider, Friction, CharacterBody) {
    (
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(
                spawn_settings.radius(),
                spawn_settings.straight_height(),
            )),
            material: material_handle,
            transform: Transform::from_xyz(0.0, spawn_settings.half_body_height(), 0.0),
            ..default()
        },
        Collider::capsule_y(
            spawn_settings.straight_height() / 2.0,
            spawn_settings.radius(),
        ),
        Friction::coefficient(spawn_settings.friction),
        CharacterBody,
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
            mesh: meshes.add(Cuboid::from_size(Vec3::splat(head_size))),
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
