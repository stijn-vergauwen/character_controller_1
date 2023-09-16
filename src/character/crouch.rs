use bevy::prelude::*;

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
    // TODO: update the crouching variable

    for mut crouch in characters.iter_mut() {
        crouch.crouching = crouch.has_crouch_input;

        println!(
            "Character is {}crouching",
            if crouch.crouching { "" } else { "not " }
        );
    }
}

fn update_body_height() {
    // TODO: set the body height and position based on crouching value
}
