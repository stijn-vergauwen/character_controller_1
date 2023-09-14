use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::config::CharacterConfig;

pub struct CharacterJumpPlugin;

impl Plugin for CharacterJumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, jump_character);
    }
}

#[derive(Component)]
pub struct CharacterJump {
    pub has_jump_input: bool,
}

impl CharacterJump {
    pub fn new() -> Self {
        Self {
            has_jump_input: false,
        }
    }
}

fn jump_character(
    mut characters: Query<(&mut ExternalImpulse, &mut CharacterJump, &CharacterConfig)>,
) {
    for (mut impulse, mut jump, config) in characters
        .iter_mut()
        .filter(|(_, jump, _)| jump.has_jump_input)
    {
        jump.has_jump_input = false;
        impulse.impulse = Vec3::Y * config.jump_strength;
    }
}
