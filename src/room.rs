use crate::tile::{Tile, TileType};
use crate::vector2::Vector2;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Room {
    pub tiles: Vec<Tile>,
    pub exits: Vec<Vector2>,
    pub pos: Vector2,
    pub id: String,
}

impl Room {
    pub fn prefab(id: &str) -> Self {
        match id {
            "c_room_0" => Room {
                id: "c_room_0".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 4.0, y: 2.0 }, Vector2 { x: 2.0, y: 4.0 }],
                ..default()
            },
            "c_room_1" => Room {
                id: "c_room_1".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 0.0, y: 2.0 }, Vector2 { x: 2.0, y: 4.0 }],
                ..default()
            },
            "c_room_2" => Room {
                id: "c_room_2".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 2.0, y: 0.0 }, Vector2 { x: 0.0, y: 2.0 }],
                ..default()
            },
            "c_room_3" => Room {
                id: "c_room_3".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 2.0, y: 0.0 }, Vector2 { x: 4.0, y: 2.0 }],
                ..default()
            },
            "d_room_0" => Room {
                id: "d_room_0".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 0.0, y: 2.0 }],
                ..default()
            },
            "d_room_1" => Room {
                id: "d_room_1".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 2.0, y: 0.0 }],
                ..default()
            },
            "d_room_2" => Room {
                id: "d_room_2".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 4.0, y: 2.0 }],
                ..default()
            },
            "d_room_3" => Room {
                id: "d_room_3".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 2.0, y: 4.0 }],
                ..default()
            },
            "exit" => Room {
                id: "exit".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Exit,
                        texture_path: "exit_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![
                    Vector2 { x: 2.0, y: 0.0 },
                    Vector2 { x: 0.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 2.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                ],
                ..default()
            },
            "m_room_0" => Room {
                id: "m_room_0".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![
                    Vector2 { x: 2.0, y: 0.0 },
                    Vector2 { x: 0.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 2.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                ],
                ..default()
            },
            "m_room_1" => Room {
                id: "m_room_1".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![
                    Vector2 { x: 2.0, y: 0.0 },
                    Vector2 { x: 0.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 2.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                ],
                ..default()
            },
            "m_room_2" => Room {
                id: "m_room_2".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![
                    Vector2 { x: 0.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 2.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                ],
                ..default()
            },
            "m_room_3" => Room {
                id: "m_room_3".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![
                    Vector2 { x: 2.0, y: 0.0 },
                    Vector2 { x: 0.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 2.0 },
                ],
                ..default()
            },
            "p_room_blue" => Room {
                id: "p_room_blue".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::PortalA,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 1.0, y: 2.0 }, Vector2 { x: 4.0, y: 2.0 }],
                ..default()
            },
            "p_room_orange" => Room {
                id: "p_room_orange".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::PortalB,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![Vector2 { x: 0.0, y: 2.0 }, Vector2 { x: 3.0, y: 2.0 }],
                ..default()
            },
            "spawn" => Room {
                id: "spawn".to_string(),
                tiles: vec![
                    Tile {
                        pos: Vector2 { x: 0.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 0.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 0.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 1.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 1.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 2.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 3.0 },
                        tile_type: TileType::Spawn,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 3.0 },
                        tile_type: TileType::Brush,
                        texture_path: "path_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 3.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 0.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 1.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 2.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 3.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                    Tile {
                        pos: Vector2 { x: 4.0, y: 4.0 },
                        tile_type: TileType::Clip,
                        texture_path: "wall_32x32.png".to_string(),
                    },
                ],
                exits: vec![
                    Vector2 { x: 2.0, y: 0.0 },
                    Vector2 { x: 0.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 2.0 },
                ],
                ..default()
            },
            _ => Room::default(),
        }
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Room(id: {}, pos: {}, tiles: {}, exits: {})",
            self.id,
            self.pos,
            self.tiles.len(),
            self.exits.len()
        )
    }
}

impl Default for Room {
    fn default() -> Self {
        Self {
            tiles: vec![],
            exits: vec![],
            pos: Vector2 { x: 0.0, y: 0.0 },
            id: "invalid".to_string(),
        }
    }
}
