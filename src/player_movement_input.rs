use bevy::prelude::*;

pub struct PlayerMovementInputPlugin;

impl Plugin for PlayerMovementInputPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

// This plugin updates the character input if a PlayerMovementInput component is added, this component hold config like keybinds