use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Copy, Debug, Component)]
pub struct Velocity {
    pub accel: Vec2,
    pub vel: Vec2,
    pub friction: f32,
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Velocity(accel: {}, vel: {}, friction: {})",
            self.accel, self.vel, self.friction
        )
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            accel: Vec2::default(),
            vel: Vec2::default(),
            friction: 8.0,
        }
    }
}
