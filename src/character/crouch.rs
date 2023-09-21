use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::CharacterBody;

pub struct CharacterCrouchPlugin;

impl Plugin for CharacterCrouchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_crouch, update_body_height));
    }
}

/// Warning! This component doesn't work properly and is in an uncomplete state.
/// 
/// Maybe wait for the next iteration to use this :P.
#[derive(Component)]
pub struct CharacterCrouch {
    pub has_crouch_input: bool,
    crouching: bool,
    lerp_value: f32,
}

impl CharacterCrouch {
    pub fn new() -> Self {
        Self {
            has_crouch_input: false,
            crouching: false,
            lerp_value: 0.0,
        }
    }

    /// Toggles the `has_crouch_input` field and returns the new value
    pub fn toggle_crouch_input(&mut self) -> bool {
        self.has_crouch_input = !self.has_crouch_input;
        self.has_crouch_input
    }
}

fn update_crouch(mut characters: Query<&mut CharacterCrouch>, time: Res<Time>) {
    for mut crouch in characters.iter_mut() {
        let lerp_change_per_second = 6.0;
        let delta_lerp = match crouch.has_crouch_input {
            true => lerp_change_per_second * time.delta_seconds(),
            false => -lerp_change_per_second * time.delta_seconds(),
        };

        crouch.lerp_value = (crouch.lerp_value + delta_lerp).clamp(0.0, 1.0);
        crouch.crouching = crouch.lerp_value > 0.0;

        // println!(
        //     "Crouching {}%",
        //     crouch.lerp_value * 100.0
        // );
    }
}

fn update_body_height(
    characters: Query<&CharacterCrouch>,
    mut character_bodies: Query<
        (&mut Collider, &mut Transform, &Handle<Mesh>, &Parent),
        With<CharacterBody>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut collider, mut transform, mesh_handle, parent) in character_bodies.iter_mut() {
        if let Ok(crouch) = characters.get(parent.get()) {
            let target_body_height = lerp(0.8, 0.4, crouch.lerp_value);

            let current_y = transform.translation.y;
            let target_y = 1.2 - target_body_height / 2.0;

            if current_y != target_y {
                transform.translation.y = target_y;
                // println!("Height: {}", transform.translation.y);

                if let Some(mesh) = meshes.get_mut(mesh_handle) {
                    *mesh = shape::Capsule {
                        depth: target_body_height,
                        radius: 0.4,
                        ..default()
                    }
                    .into();
                }

                if let Some(mut capsule) = collider.as_capsule_mut() {
                    capsule.set_segment(Vec3::NEG_Y * (target_body_height - 0.4), Vec3::Y * 0.4);
                }
            }
        }
    }
}

fn lerp(start: f32, end: f32, value: f32) -> f32 {
    start + (end - start) * value
}
