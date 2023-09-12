use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::character::Character;

pub struct PlayerMovementInputPlugin;

impl Plugin for PlayerMovementInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_character_movement_input,
                update_character_rotation_input,
            ),
        );
    }
}

// This plugin updates the character input if a PlayerMovementInput component is added, this component hold config like keybinds

#[derive(Component)]
pub struct PlayerMovementInput {
    keybinds: MovementKeybinds,
    hold_to_run: bool,
    hold_to_crouch: bool,
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
    forward_key: KeyCode,
    back_key: KeyCode,
    left_key: KeyCode,
    right_key: KeyCode,
    run_key: KeyCode,
    jump_key: KeyCode,
    crouch_key: KeyCode,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward_key: KeyCode::W,
            back_key: KeyCode::S,
            left_key: KeyCode::A,
            right_key: KeyCode::D,
            run_key: KeyCode::ShiftLeft,
            jump_key: KeyCode::Space,
            crouch_key: KeyCode::ControlLeft,
        }
    }
}

fn update_character_movement_input(
    mut character_query: Query<(&PlayerMovementInput, &mut Character)>,
    input: Res<Input<KeyCode>>,
) {
    for (movement, mut character) in character_query.iter_mut() {
        let direction = walk_direction_from_input(&movement.keybinds, &input);

        character.set_movement_input(direction);
    }
}

fn update_character_rotation_input(
    mut character_query: Query<&mut Character, With<PlayerMovementInput>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let sum = mouse_motion
        .iter()
        .fold(Vec2::ZERO, |sum, motion| sum + motion.delta);

    let as_rotation = Vec3::new(-sum.y, -sum.x, 0.0);

    for mut character in character_query.iter_mut() {
        character.set_rotation_input(as_rotation);
    }
}

fn walk_direction_from_input(keybinds: &MovementKeybinds, input: &Res<Input<KeyCode>>) -> Vec3 {
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
