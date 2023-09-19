pub mod camera;
pub mod config;
pub mod crouch;
mod interaction;
pub mod jump;
mod movement;
mod rotation;
pub mod spawner;

use bevy::prelude::*;

use self::{
    crouch::CharacterCrouchPlugin, jump::CharacterJumpPlugin, movement::CharacterMovementPlugin,
    rotation::CharacterRotationPlugin,
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
            CharacterCrouchPlugin,
        ));
    }
}

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
    pub movement_force: Vec3,
    pub corrective_force: Vec3,
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
            movement_force: Vec3::ZERO,
            corrective_force: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct CharacterHead;

#[derive(Component)]
pub struct CharacterBody;
