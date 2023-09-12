use bevy::prelude::*;

use super::{Character, CharacterHead};

pub struct CharacterRotationPlugin;

impl Plugin for CharacterRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rotate_character_horizontally, rotate_character_vertically));
    }
}

fn rotate_character_horizontally(mut characters: Query<(&mut Transform, &Character)>) {
    for (mut transform, character) in characters.iter_mut() {
        println!("rotate by {}", character.rotation_input.y * character.turn_strength);
        transform.rotate_local(Quat::from_axis_angle(
            Vec3::Y,
            character.rotation_input.y * character.turn_strength,
        ));
    }
}

pub fn rotate_character_vertically(
    mut head_query: Query<(&mut Transform, &Parent), With<CharacterHead>>,
    character_query: Query<&Character>,
) {
    for (mut transform, parent) in head_query.iter_mut() {
        if let Ok(character) = character_query.get(parent.get()) {
            transform.rotate_local(Quat::from_axis_angle(
                Vec3::X,
                character.rotation_input.x * character.turn_strength,
            ));
        }
    }
}