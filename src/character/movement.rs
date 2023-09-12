use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Character;

pub struct CharacterMovementPlugin;

impl Plugin for CharacterMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_characters);
    }
}

fn move_characters(mut characters: Query<(&mut ExternalForce, &Character, &Transform)>) {
    for (mut force, character, transform) in characters.iter_mut() {
        println!("{character:#?}");
        // TODO: rotate movement direction by the normal of the current ground object
        let direction = transform.rotation * character.movement_input;

        force.force = direction * character.get_movement_strength();
    }
}
