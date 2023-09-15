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
                move_character_with_grounded,
                limit_character_speed.after(move_character),
                stop_running_if_no_movement_input,
            ),
        );
    }
}

fn move_character_with_grounded(
    mut characters: Query<(
        &mut ExternalForce,
        &Character,
        &CharacterConfig,
        &Transform,
        &Grounded,
    )>,
) {
    for (mut force, character, config, transform, grounded) in characters
        .iter_mut()
        .filter(|(_, character, _, _, _)| character.is_active)
    {
        // TODO: rotate movement direction by the normal of the current ground object
        let direction = transform.rotation * character.movement_input;

        force.force =
            direction * config.get_movement_strength(grounded.is_grounded(), character.is_running);
    }
}

// TODO: how would you refactor out the dependency on the Grounded component?
fn move_character(
    mut characters: Query<
        (&mut ExternalForce, &Character, &CharacterConfig, &Transform),
        Without<Grounded>,
    >,
) {
    for (mut force, character, config, transform) in characters
        .iter_mut()
        .filter(|(_, character, _, _)| character.is_active)
    {
        // TODO: rotate movement direction by the normal of the current ground object
        let direction = transform.rotation * character.movement_input;
        let is_grounded = true;

        force.force = direction * config.get_movement_strength(is_grounded, character.is_running);
    }
}

// TODO: this system is probably redundant once the character's 'active correcting force' is implemented
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
