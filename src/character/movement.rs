use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{Character, config::CharacterConfig};

pub struct CharacterMovementPlugin;

impl Plugin for CharacterMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_character);
    }
}

fn move_character(mut characters: Query<(&mut ExternalForce, &Character, &CharacterConfig, &Transform)>) {
    for (mut force, character, config, transform) in characters.iter_mut() {
        // TODO: rotate movement direction by the normal of the current ground object
        // TODO: stop accelerating at max walk / run speed
        let direction = transform.rotation * character.movement_input;

        force.force = direction * config.get_movement_strength(character);
    }
}