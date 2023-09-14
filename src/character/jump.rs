use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

fn jump_character(characters: Query<(&mut ExternalImpulse, &CharacterJump)>) {
    // TODO: implement jump functionality
    for (_, jump) in characters.iter() {
        println!("Has jump input: {}", jump.has_jump_input);
    }
}
