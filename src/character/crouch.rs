use bevy::prelude::*;

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

fn update_crouch(
    characters: Query<&CharacterCrouch>,
    mut character_bodies: Query<(&mut CharacterBody, &Parent)>,
) {
    // for (body, parent) in characters.iter() {
    //     if let Ok(body) = character_bodies.get(children.ite) {

    //     }
    // }

    // TODO: update the crouching variable
}

fn update_body_height() {
    // TODO: set the body height and position based on crouching value
}
