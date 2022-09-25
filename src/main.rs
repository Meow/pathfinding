mod equipment;
mod inventory;
mod item;
mod map;
mod player;
mod room;
mod tile;
mod traits;
mod vector2;

use crate::map::Map;
use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let map = Map::random();

    for room in &map.rooms {
        for tile in &room.tiles {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&tile.texture_path),
                    transform: Transform::from_xyz(
                        room.pos.x * 32.0 + tile.pos.x * 32.0,
                        room.pos.y * 32.0 + tile.pos.y * 32.0,
                        0.,
                    ),
                    ..default()
                })
                .insert(tile.clone());
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
