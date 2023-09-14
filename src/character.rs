pub mod config;
mod interaction;
pub mod jump;
mod movement;
mod rotation;
pub mod spawner;

use bevy::prelude::*;

use self::{
    jump::CharacterJumpPlugin, movement::CharacterMovementPlugin, rotation::CharacterRotationPlugin,
};

/*
    Standards to hold myself to in this project:

    - Data that affects behaviour (like input keybinds or move speed) should be stored in the components, not in constants
    - Every entity spawned should have a Name component
    - The plugin should panic nowhere, and not use the get_single functions.
*/

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CharacterMovementPlugin,
            CharacterRotationPlugin,
            CharacterJumpPlugin,
        ));
    }
}

// TODO: make character camera it's own component, to allow no cam or third person cam

// TODO: when standing on an object, use it's normal direction to align the characters movement forces

// TODO: add a correcting force that pushes the character velocity to it's move input

/// The main character component, holds state and current inputs.
///
/// NOTE: To spawn a character it is recommended to use the spawner module.
#[derive(Component, Debug)]
pub struct Character {
    pub is_active: bool,
    pub is_running: bool,
    pub movement_input: Vec3,
    pub rotation_input: Vec3,
}

impl Character {
    pub fn toggle_running(&mut self) {
        self.is_running = !self.is_running;
    }

    /// Sets `is_active` to true.
    pub fn activate(&mut self) {
        self.is_active = true;
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            is_active: false,
            is_running: false,
            movement_input: Vec3::ZERO,
            rotation_input: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct CharacterHead;
