use bevy::prelude::*;

use super::{config::CharacterConfig, Character, CharacterHead};

pub struct CharacterRotationPlugin;

impl Plugin for CharacterRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (rotate_character_horizontally, rotate_character_vertically),
        );
    }
}

fn rotate_character_horizontally(
    mut characters: Query<(&mut Transform, &Character, &CharacterConfig)>,
) {
    for (mut transform, character, config) in characters
        .iter_mut()
        .filter(|(_, character, _)| character.is_active)
    {
        transform.rotate_local(Quat::from_axis_angle(
            Vec3::Y,
            character.rotation_input.y * config.turn_speed,
        ));
    }
}

pub fn rotate_character_vertically(
    mut character_heads: Query<(&mut Transform, &Parent), With<CharacterHead>>,
    characters: Query<(&Character, &CharacterConfig)>,
) {
    for (mut transform, parent) in character_heads.iter_mut() {
        if let Ok((character, config)) = characters.get(parent.get()) {
            if character.is_active {
                let vertical_angle = transform.rotation.to_scaled_axis().x;
                let angle_limit_rad = config.vertical_rotation_limit_degrees.to_radians();
                let new_angle = (vertical_angle + character.rotation_input.x * config.turn_speed)
                    .clamp(-angle_limit_rad, angle_limit_rad);

                transform.rotation = Quat::from_axis_angle(Vec3::X, new_angle);
            }
        }
    }
}
