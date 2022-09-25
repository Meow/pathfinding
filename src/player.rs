use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Player {
    max_items: u32,
    attack: f32,
    defense: f32,
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            max_items: 10,
            attack: 1.0,
            defense: 1.0,
            speed: 1.0,
        }
    }
}
