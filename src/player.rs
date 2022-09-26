use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Player {
    pub max_items: u32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub max_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            max_items: 10,
            attack: 1.0,
            defense: 1.0,
            speed: 1.0,
            max_speed: 100.0,
        }
    }
}
