mod interaction;
mod jump;
mod movement;
mod rotation;
pub mod spawner;

use bevy::prelude::*;

use self::{movement::CharacterMovementPlugin, rotation::CharacterRotationPlugin};

/*
    Standards to hold myself to in this project:

    - Data that affects behaviour (like input keybinds or move speed) should be stored in the components, not in constants
    - Every entity spawned should have a Name component
    - The plugin should panic nowhere, and not use the get_single functions.
*/

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CharacterMovementPlugin, CharacterRotationPlugin));
    }
}

// TODO: make separate component for all character configuration, like movement strengths

// TODO: move on_ground to it's own general component that checks if things are grounded

// TODO: make character camera it's own component, to allow no cam or third person cam.
// TODO: store Option<camera id> on character head component, I think this is a good way to update movement without transform hierarchy

// TODO: when standing on an object, use it's normal direction to align the characters movement forces

#[derive(Component, Debug)]
pub struct Character {
    pub is_active: bool,
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

    /// Sets the movement input of this [`Character`] to the given value, or `Vec3::ZERO` if this character is inactive.
    pub fn set_movement_input(&mut self, value: Vec3) {
        self.movement_input = if self.is_active { value } else { Vec3::ZERO };
    }

    /// Sets the rotation input of this [`Character`] to the given value, or `Vec3::ZERO` if this character is inactive.
    pub fn set_rotation_input(&mut self, value: Vec3) {
        self.rotation_input = if self.is_active { value } else { Vec3::ZERO };
    }

    pub fn get_movement_strength(&self) -> f32 {
        match self.is_active && self.is_running {
            false => self.walk_strength,
            true => self.run_strength,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            is_active: false,
            is_running: false,
            on_ground: false,
            movement_input: Vec3::ZERO,
            rotation_input: Vec3::ZERO,
            walk_strength: 8.0,
            run_strength: 13.0,
            jump_strength: 3.0,
            // TODO: make this turn strength value less weird
            turn_strength: 0.0007,
        }
    }
}

#[derive(Component)]
pub struct CharacterHead;
