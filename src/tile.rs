use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TileType {
    Spawn,
    Exit,
    Clip,
    Brush,
    Item,
    PortalA,
    PortalB,
    None,
}

#[derive(Clone, Debug, Component)]
pub struct Tile {
    pub pos: Vec2,
    pub tile_type: TileType,
    pub texture_path: String,
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

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::None,
            texture_path: "path_32x32.png".to_string(),
            pos: Vec2 { x: 0.0, y: 0.0 },
        }
    }
}
