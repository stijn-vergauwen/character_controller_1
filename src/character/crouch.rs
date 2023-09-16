use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::CharacterBody;

pub struct CharacterCrouchPlugin;

impl Plugin for CharacterCrouchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_crouch, update_body_height));
    }
}

#[derive(Component)]
pub struct CharacterCrouch {
    pub has_crouch_input: bool,
    crouching: bool,
}

impl CharacterCrouch {
    pub fn new() -> Self {
        Self {
            has_crouch_input: false,
            crouching: false,
        }
    }

    /// Toggles the `has_crouch_input` field and returns the new value
    pub fn toggle_crouch_input(&mut self) -> bool {
        self.has_crouch_input = !self.has_crouch_input;
        self.has_crouch_input
    }
}

fn update_crouch(mut characters: Query<&mut CharacterCrouch>) {
    for mut crouch in characters.iter_mut() {
        crouch.crouching = crouch.has_crouch_input;

        println!(
            "Character is {}crouching",
            if crouch.crouching { "" } else { "not " }
        );
    }
}

fn update_body_height(
    characters: Query<&CharacterCrouch>,
    mut character_bodies: Query<(&mut Collider, &Handle<Mesh>, &Parent), With<CharacterBody>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut collider, mesh_handle, parent) in character_bodies.iter_mut() {
        if let Ok(crouch) = characters.get(parent.get()) {
            let target_body_height = match crouch.crouching {
                true => 0.0,
                false => 0.8,
            };

            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                *mesh = shape::Capsule {
                    depth: target_body_height,
                    radius: 0.4,
                    ..default()
                }
                .into();
            }

            if let Some(mut capsule) = collider.as_capsule_mut() {
                capsule.set_segment(
                    Vec3::NEG_Y * target_body_height / 2.0,
                    Vec3::Y * target_body_height / 2.0,
                );
            }
        }
    }
}
