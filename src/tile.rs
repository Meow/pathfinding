use crate::vector2::Vector2;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub enum TileType {
    Spawn,
    Exit,
    Clip,
    Brush,
    Item,
    None,
}

#[derive(Clone, Debug, Component)]
pub struct Tile {
    pos: Vector2,
    tile_type: TileType,
    texture_path: String,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileType::Spawn => write!(f, "TileType::Spawn"),
            TileType::Exit => write!(f, "TileType::Exit"),
            TileType::Clip => write!(f, "TileType::Clip"),
            TileType::Brush => write!(f, "TileType::Brush"),
            TileType::Item => write!(f, "TileType::Item"),
            TileType::None => write!(f, "TileType::None"),
            _ => write!(f, "Invalid TileType"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile(pos: {}, type: {})", self.pos, self.tile_type)
    }
}
