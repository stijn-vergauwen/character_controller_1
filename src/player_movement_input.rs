use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    character::{crouch::CharacterCrouch, jump::CharacterJump, Character},
    grounded::Grounded,
};

pub struct PlayerMovementInputPlugin;

impl Plugin for PlayerMovementInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_character_movement_input,
                update_character_rotation_input,
                update_character_running,
                update_character_jump_input,
                update_character_crouch_input,
            ),
        );
    }
}

#[derive(Component)]
pub struct PlayerMovementInput {
    pub keybinds: MovementKeybinds,
    pub hold_to_run: bool,
    pub hold_to_crouch: bool,
}

impl Default for PlayerMovementInput {
    fn default() -> Self {
        Self {
            keybinds: MovementKeybinds::default(),
            hold_to_run: true,
            hold_to_crouch: true,
        }
    }
}

pub struct MovementKeybinds {
    pub forward_key: KeyCode,
    pub back_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub run_key: KeyCode,
    pub jump_key: KeyCode,
    pub crouch_key: KeyCode,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward_key: KeyCode::KeyW,
            back_key: KeyCode::KeyS,
            left_key: KeyCode::KeyA,
            right_key: KeyCode::KeyD,
            run_key: KeyCode::ShiftLeft,
            jump_key: KeyCode::Space,
            crouch_key: KeyCode::ControlLeft,
        }
    }
}

fn update_character_movement_input(
    mut characters: Query<(&PlayerMovementInput, &mut Character)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (movement, mut character) in characters.iter_mut() {
        let direction = walk_direction_from_input(&movement.keybinds, &input);

        character.movement_input = direction;
    }
}

fn update_character_rotation_input(
    mut characters: Query<&mut Character, With<PlayerMovementInput>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let sum = mouse_motion
        .read()
        .fold(Vec2::ZERO, |sum, motion| sum + motion.delta);

    let as_rotation = Vec3::new(-sum.y, -sum.x, 0.0);

    for mut character in characters.iter_mut() {
        character.rotation_input = as_rotation;
    }
}

fn update_character_running(
    mut characters: Query<(&PlayerMovementInput, &mut Character)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (movement, mut character) in characters.iter_mut() {
        if movement.hold_to_run {
            if input.pressed(movement.keybinds.run_key) != character.is_running {
                character.toggle_running();
            }
        } else {
            if input.just_pressed(movement.keybinds.run_key) {
                character.toggle_running();
            }
        }
    }
}

fn update_character_jump_input(
    mut characters: Query<(&PlayerMovementInput, &mut CharacterJump, &Grounded)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (movement, mut jump, grounded) in characters.iter_mut() {
        if input.just_pressed(movement.keybinds.jump_key) && grounded.is_grounded() {
            jump.has_jump_input = true;
        }
    }
}

fn update_character_crouch_input(
    mut characters: Query<(&PlayerMovementInput, &mut CharacterCrouch)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (movement, mut crouch) in characters.iter_mut() {
        if movement.hold_to_crouch {
            if input.pressed(movement.keybinds.crouch_key) != crouch.has_crouch_input {
                crouch.toggle_crouch_input();
            }
        } else {
            if input.just_pressed(movement.keybinds.crouch_key) {
                crouch.toggle_crouch_input();
            }
        }
    }
}

fn walk_direction_from_input(
    keybinds: &MovementKeybinds,
    input: &Res<ButtonInput<KeyCode>>,
) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if input.pressed(keybinds.forward_key) {
        direction.z -= 1.0;
    }

    if input.pressed(keybinds.back_key) {
        direction.z += 1.0;
    }

    if input.pressed(keybinds.left_key) {
        direction.x -= 1.0;
    }

    if input.pressed(keybinds.right_key) {
        direction.x += 1.0;
    }

    direction.normalize_or_zero()
}
