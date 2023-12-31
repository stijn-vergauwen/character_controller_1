pub mod camera;
pub mod config;
pub mod crouch;
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


    Things for the next iteration:

    - use newtypes to make data more descriptive, e.g. "turn speed" has a very weird value
    - set the character's mass manually and make the collider densities 0, for consistency
    - add character interaction, didn't have a clear idea of how that would look for now
    - grounded & jump components:
        - don't make grounded component optional, if the functionality already works why would you not just always use it?
        - prevent sliding when idle on a slope. either snap transform back to prev position or try putting the rigidbody to sleep
        - when the grounded check casts a shape, use that shape collision to get the normal directly, the current system causes the character to get stuck a bit on the edge of a slope
    - crouching component:
        - use a different system for crouching, either:
            - resize the collider downwards so character stays grounded
            - make alternative to resizing like splitting body in multiple parts and rotating those, which would be more accurate also
        - no magic numbers in crouching code, move these parameters to a config
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

/// The main character component, holds state and current inputs.
///
/// NOTE: To spawn a character it is recommended to use the `CharacterSpawner` helper struct.
/// 
/// `is_active` and `draw_movement_gizmos` are the only fields you might want to set manually.
#[derive(Component, Debug)]
pub struct Character {
    pub is_active: bool,
    pub is_running: bool,
    pub movement_input: Vec3,
    pub rotation_input: Vec3,
    pub movement_direction: Vec3,
    pub corrective_direction: Vec3,

    /// Visualize the movement forces acting on this character.
    /// 
    /// * Cyan = the current velocity.
    /// * Purple = the target velocity.
    /// * Red = a correcting force to reduce drifting and sliding.
    pub draw_movement_gizmos: bool,
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
            is_active: true,
            is_running: false,
            movement_input: Vec3::ZERO,
            rotation_input: Vec3::ZERO,
            movement_direction: Vec3::ZERO,
            corrective_direction: Vec3::ZERO,
            draw_movement_gizmos: false,
        }
    }
}

#[derive(Component)]
pub struct CharacterHead;

#[derive(Component)]
pub struct CharacterBody;
