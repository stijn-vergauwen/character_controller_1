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
                move_character,
                limit_character_speed.after(move_character),
                stop_running_if_no_movement_input,
            ),
        );
    }
}

// TODO: disable jump when crouching
// TODO: Make grounded component optional to work for characters without it

fn move_character(
    mut characters: Query<(
        &mut ExternalForce,
        &Character,
        &CharacterConfig,
        &Transform,
        &Grounded,
    )>,
    mut gizmos: Gizmos,
) {
    for (mut force, character, config, transform, grounded) in characters
        .iter_mut()
        .filter(|(_, character, _, _, _)| character.is_active)
    {
        let mut direction = character.movement_input;

        // TODO: fix movement input not being aligned with character transform when 'ground_normal()' return none.
        //       (tranform.rotation * movement_input happens only in ground normal calculations)

        if let Some(ground_normal) = grounded.ground_normal() {
            let delta_rotation = calculate_rotation_to_ground_normal(
                &mut gizmos,
                transform.translation,
                transform.rotation,
                ground_normal,
            );

            direction = delta_rotation * direction;
        }

        // Draw direction of movement force
        gizmos.ray(transform.translation, direction * 2.0, Color::FUCHSIA);

        // TODO: don't write result to force.force, store it in character instead.

        force.force =
            direction * config.get_movement_strength(grounded.is_grounded(), character.is_running);
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

// TODO: refactor to remove duplication
// TODO: move gizmos to dedicated system

fn calculate_rotation_to_ground_normal(
    gizmos: &mut Gizmos,
    position: Vec3,
    character_rotation: Quat,
    ground_normal: Vec3,
) -> Quat {
    let normal_rotation = looking_towards(ground_normal, Vec3::Z)
        * Quat::from_axis_angle(Vec3::X, (-90.0 as f32).to_radians());
    let delta_rotation = normal_rotation * character_rotation;

    // Draw ground normal
    draw_axis_gizmos(
        gizmos,
        position + Vec3::new(-2.0, 0.5, 0.0),
        normal_rotation,
        1.0,
    );

    // Draw adjusted character rotation
    draw_axis_gizmos(
        gizmos,
        position + Vec3::new(-2.0, 2.0, 0.0),
        character_rotation,
        1.0,
    );

    // Draw delta
    draw_axis_gizmos(
        gizmos,
        position + Vec3::new(-2.0, 3.5, 0.0),
        delta_rotation,
        1.0,
    );

    // Draw resulting orientation (should align with ground normal)
    gizmos.circle(
        position,
        delta_rotation * Vec3::Y,
        0.55,
        Color::VIOLET,
    );

    delta_rotation
}

fn draw_axis_gizmos(gizmos: &mut Gizmos, origin: Vec3, rotation: Quat, size: f32) {
    gizmos.ray(origin, rotation * Vec3::X * size, Color::RED);
    gizmos.ray(origin, rotation * Vec3::Y * size, Color::GREEN);
    gizmos.ray(origin, rotation * Vec3::Z * size, Color::BLUE);
}
