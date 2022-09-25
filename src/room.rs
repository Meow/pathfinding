use crate::tile::Tile;
use crate::vector2::Vector2;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Room {
    tiles: Vec<Tile>,
    exits: Vec<Vector2>,
    pos: Vector2,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Room(pos: {}, tiles: {}, exits: {})",
            self.pos,
            self.tiles.len(),
            self.exits.len()
        )
    }
}

impl Default for Room {
    fn default() -> Self {
        Room {
            tiles: vec![],
            exits: vec![],
            pos: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}
