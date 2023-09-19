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
                    limit_character_speed,
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

fn update_corrective_force(mut characters: Query<(&mut Character, &Velocity, &Grounded)>) {
    for (mut character, velocity, grounded) in characters
        .iter_mut()
        .filter(|(character, _, _)| character.is_active)
    {

        // TODO: move this into character config
        let strength = 1.0;

        character.corrective_force = match grounded.is_grounded() {
            true => (character.movement_force - velocity.linvel) * strength,
            false => Vec3::ZERO,
        };
    }
}

fn move_character(mut characters: Query<(&Character, &mut ExternalForce)>) {
    for (character, mut force) in characters
        .iter_mut()
        .filter(|(character, _)| character.is_active)
    {
        force.force = character.movement_force + character.corrective_force;
    }
}

// TODO: can probably be removed once the character's 'active correcting force' is implemented
fn limit_character_speed(mut characters: Query<(&mut Velocity, &Character, &CharacterConfig)>) {
    for (mut velocity, character, config) in
        characters
            .iter_mut()
            .filter(|(velocity, character, config)| {
                is_over_max_speed(
                    flatten_vector(velocity.linvel),
                    config.get_movement_speed(character.is_running),
                )
            })
    {
        // bad to have hardcoded value but I'm pretty sure this system is temporary
        let slowdown_factor = 0.2;

        let max_speed = config.get_movement_speed(character.is_running);
        let fraction_over_max = (flatten_vector(velocity.linvel).length() - max_speed) / max_speed;

        velocity.linvel *= 1.0 - fraction_over_max * slowdown_factor;
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

fn is_over_max_speed(velocity: Vec3, max_speed: f32) -> bool {
    velocity.length() > max_speed
}

/// Resets the y value of the given vector to 0.0
fn flatten_vector(vector: Vec3) -> Vec3 {
    Vec3 {
        x: vector.x,
        y: 0.0,
        z: vector.z,
    }
}

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
