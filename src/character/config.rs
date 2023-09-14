use bevy::prelude::*;

#[derive(Component)]
pub struct CharacterConfig {
    pub walk_speed: f32,
    pub walk_strength: f32,
    pub run_speed: f32,
    pub run_strength: f32,
    pub turn_speed: f32,
    pub jump_strength: f32,
    pub drag_factor: f32,

    /// What movement will be multiplied by when in the air
    pub aerial_multiplier: f32,
}

impl CharacterConfig {
    pub fn get_movement_strength(&self, is_grounded: bool, is_running: bool) -> f32 {
        let mut strength = match is_running {
            false => self.walk_strength,
            true => self.run_strength,
        };

        if !is_grounded {
            strength *= self.aerial_multiplier;
        }

        strength
    }

    pub fn get_movement_speed(&self, is_running: bool) -> f32 {
        match is_running {
            false => self.walk_speed,
            true => self.run_speed,
        }
    }
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            walk_strength: 7.0,
            run_speed: 10.0,
            run_strength: 15.0,
            jump_strength: 5.0,
            // TODO: make this turn strength value less wierd
            turn_speed: 0.0007,
            drag_factor: 0.5,
            aerial_multiplier: 0.5,
        }
    }
}
