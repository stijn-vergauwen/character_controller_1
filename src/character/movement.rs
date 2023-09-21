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
                    update_movement_direction,
                    update_corrective_direction,
                    move_character,
                )
                    .chain(),
                stop_running_if_no_movement_input,
                draw_gizmos,
            ),
        );
    }
}

fn update_movement_direction(
    mut characters: Query<(&mut Character, &Transform, Option<&Grounded>)>,
) {
    for (mut character, transform, grounded) in characters
        .iter_mut()
        .filter(|(character, _, _)| character.is_active)
    {
        let ground_rotation = get_ground_rotation(grounded).unwrap_or(Quat::IDENTITY);

        let movement_direction = align_direction_to_ground(
            ground_rotation,
            transform.rotation,
            character.movement_input,
        );

        character.movement_direction = movement_direction;
    }
}

fn update_corrective_direction(
    mut characters: Query<(
        &mut Character,
        &CharacterConfig,
        &Velocity,
        Option<&Grounded>,
    )>,
) {
    for (mut character, config, velocity, grounded) in characters
        .iter_mut()
        .filter(|(character, _, _, _)| character.is_active)
    {
        let treshold = 0.00001;
        let delta = (character.movement_direction
            * config.get_movement_speed(character.is_running))
            - velocity.linvel;

        character.corrective_direction = if delta.length() > treshold {
            match grounded {
                Some(grounded) => match grounded.ground_rotation() {
                    Some(rotation) => rotation * vector_without_y(delta).normalize_or_zero(),
                    None => Vec3::ZERO,
                },
                None => vector_without_y(delta).normalize_or_zero(),
            }
        } else {
            Vec3::ZERO
        }
    }
}

fn move_character(
    mut characters: Query<(
        &mut ExternalForce,
        &Character,
        &CharacterConfig,
        Option<&Grounded>,
    )>,
) {
    for (mut force, character, config, grounded) in characters
        .iter_mut()
        .filter(|(_, character, _, _)| character.is_active)
    {
        let combined_direction = character.movement_direction + character.corrective_direction;
        let is_grounded = match grounded {
            Some(grounded) => grounded.is_grounded(),
            None => true,
        };
        let strength = config.get_movement_strength(is_grounded, character.is_running);

        force.force = combined_direction * strength;
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

fn draw_gizmos(
    characters: Query<(
        &Character,
        &GlobalTransform,
        &Velocity,
        &CharacterConfig,
        Option<&Grounded>,
    )>,
    mut gizmos: Gizmos,
) {
    let position_offset = Vec3::Y * 0.05;
    let current_velocity_color = Color::CYAN;
    let target_velocity_color = Color::FUCHSIA;
    let corrective_force_color = Color::RED;
    let length = 0.4;

    for (character, global_transform, velocity, config, grounded) in characters
        .iter()
        .filter(|(character, _, _, _, _)| character.draw_movement_gizmos)
    {
        let position = global_transform.translation() + position_offset;
        let is_running = character.is_running;
        let is_grounded = match grounded {
            Some(grounded) => grounded.is_grounded(),
            None => true,
        };

        gizmos.ray(position, velocity.linvel * length, current_velocity_color);

        gizmos.ray(
            position,
            character.movement_direction * length * config.get_movement_speed(is_running),
            target_velocity_color,
        );

        gizmos.ray(
            position,
            character.corrective_direction
                * length
                * config.get_movement_strength(is_grounded, is_running),
            corrective_force_color,
        );
    }
}

// Utilities

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

fn get_ground_rotation(grounded: Option<&Grounded>) -> Option<Quat> {
    grounded?.ground_rotation()
}
