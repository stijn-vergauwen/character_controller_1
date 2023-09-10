mod input;
mod interaction;
mod jump;
mod movement;
mod rotation;
mod spawner;

use bevy::prelude::*;

/*
    Standards to hold myself to in this project:

    - Data that affects behaviour (like input keybinds or move speed) should be stored in the components, not in constants
    - Every entity spawned should have a Name component
*/

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}
