mod interaction;
mod jump;
mod movement;
mod rotation;
pub mod spawner;

use bevy::prelude::*;

/*
    Standards to hold myself to in this project:

    - Data that affects behaviour (like input keybinds or move speed) should be stored in the components, not in constants
    - Every entity spawned should have a Name component
*/

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

// TODO: make component for user input, if added set character input using user input, otherwise leave it (replaces bool)
// TODO: make separate component for all character configuration, like movement strengths

// TODO: move on_ground to it's own general component that checks if things are grounded

// TODO: make character camera it's own component, to allow no cam or third person cam.

// TODO: store Option<camera id> on character head component, I think this is a good way to update movement without transform hierarchy

#[derive(Component)]
pub struct Character {
    is_active: bool,
    is_running: bool,
    on_ground: bool,
    movement_input: Vec3,
    rotation_input: Vec3,
    walk_strength: f32,
    run_strength: f32,
    jump_strength: f32,
    turn_strength: f32,
}

impl Character {
    fn new(walk_strength: f32, run_strength: f32, jump_strength: f32, turn_strength: f32) -> Self {
        Self {
            is_active: false,
            is_running: false,
            on_ground: false,
            movement_input: Vec3::ZERO,
            rotation_input: Vec3::ZERO,
            walk_strength,
            run_strength,
            jump_strength,
            turn_strength,
        }
    }

    /// Sets `is_active` to true.
    fn activate(&mut self) {
        self.is_active = true;
    }


}
