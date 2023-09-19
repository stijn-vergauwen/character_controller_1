use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::grounded::Grounded;

use super::{config::CharacterConfig, Character};

pub struct CharacterMovementPlugin;

impl Plugin for CharacterMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    update_movement_force,
                    update_corrective_force,
                    move_character,
                )
                    .chain(),
                stop_running_if_no_movement_input,
                draw_gizmos,
            ),
        );
    }
}

// TODO: disable jump when crouching
// TODO: Make grounded component optional to work for characters without it

// TODO: make this the movement target, not the actual force

fn update_movement_force(
    mut characters: Query<(&mut Character, &CharacterConfig, &Transform, &Grounded)>,
) {
    for (mut character, config, transform, grounded) in characters
        .iter_mut()
        .filter(|(character, _, _, _)| character.is_active)
    {
        let ground_rotation = match grounded.ground_normal() {
            Some(normal) => ground_normal_as_rotation(normal),
            None => Quat::IDENTITY,
        };

        let movement_direction = align_direction_to_ground(
            ground_rotation,
            transform.rotation,
            character.movement_input,
        );

        character.movement_force = movement_direction
            * config.get_movement_strength(grounded.is_grounded(), character.is_running);
    }
}

// TODO: rename corrective force to movement force

fn update_corrective_force(
    mut characters: Query<(&mut Character, &CharacterConfig, &Velocity, &Grounded)>,
) {
    for (mut character, config, velocity, grounded) in characters
        .iter_mut()
        .filter(|(character, _, _, _)| character.is_active)
    {
        let delta = character.movement_force - velocity.linvel;

        character.corrective_force = if delta.length() > 0.00001 {
            delta.normalize_or_zero()
                * config.get_movement_strength(grounded.is_grounded(), character.is_running)
        } else {
            Vec3::ZERO
        }
    }
}

fn move_character(mut characters: Query<(&Character, &mut ExternalForce)>) {
    for (character, mut force) in characters
        .iter_mut()
        .filter(|(character, _)| character.is_active)
    {
        force.force = character.corrective_force;
    }
}

fn stop_running_if_no_movement_input(mut characters: Query<&mut Character>) {
    for mut character in characters
        .iter_mut()
        .filter(|character| character.is_running && character.movement_input == Vec3::ZERO)
    {
        character.toggle_running();
    }
}

// Gizmos

fn draw_gizmos(characters: Query<(&Character, &GlobalTransform, &Velocity)>, mut gizmos: Gizmos) {
    let position_offset = Vec3::Y * 0.05;
    let current_velocity_color = Color::CYAN;
    let target_velocity_color = Color::FUCHSIA;
    let corrective_force_color = Color::RED;
    let length = 0.4;

    for (character, global_transform, velocity) in characters.iter() {
        let position = global_transform.translation() + position_offset;

        gizmos.ray(position, velocity.linvel * length, current_velocity_color);

        gizmos.ray(
            position,
            character.movement_force * length,
            target_velocity_color,
        );

        gizmos.ray(
            position,
            character.corrective_force * length,
            corrective_force_color,
        );
    }
}

// Utilities

fn looking_towards(direction: Vec3, up: Vec3) -> Quat {
    let back = -direction.try_normalize().unwrap_or(Vec3::NEG_Z);
    let up = up.try_normalize().unwrap_or(Vec3::Y);
    let right = up
        .cross(back)
        .try_normalize()
        .unwrap_or_else(|| up.any_orthonormal_vector());
    let up = back.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, back))
}

/// Returns the ground normal direction as a quaternion where up is the Y axis.
fn ground_normal_as_rotation(normal: Vec3) -> Quat {
    looking_towards(normal, Vec3::Z) * Quat::from_axis_angle(Vec3::X, (-90.0 as f32).to_radians())
}

/// Returns the direction aligned with the ground and turned to the characters rotation.
fn align_direction_to_ground(
    ground_rotation: Quat,
    character_rotation: Quat,
    direction: Vec3,
) -> Vec3 {
    ground_rotation * character_rotation * direction
}

/// Returns the vector with it's Y component set to 0.
fn vector_without_y(vector: Vec3) -> Vec3 {
    Vec3::new(vector.x, 0.0, vector.z)
}
