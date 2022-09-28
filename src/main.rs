mod equipment;
mod inventory;
mod item;
mod map;
mod player;
mod room;
mod tile;
mod traits;
mod velocity;

use crate::{
    inventory::Inventory, inventory::SortingDirection, inventory::SortingField, map::Map,
    player::Player, tile::Tile, tile::TileType, velocity::Velocity,
};
use bevy::prelude::*;
use bevy::sprite::{collide_aabb, collide_aabb::Collision};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            scale: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            translation: Vec3 {
                z: 5.0,
                ..default()
            },
            ..default()
        },
        ..default()
    });

    let map = Map::random();
    let mut spawn_pos = Vec2::default();

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

            if tile.tile_type == TileType::Spawn {
                spawn_pos = (room.pos + tile.pos) * 32.0;
            }
        }
    }

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player_new_32x32.png"),
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y + 6.0, 1.0),
            ..default()
        })
        .insert(Player::default())
        .insert(Inventory::default())
        .insert(Velocity::default());
}

fn set_msaa(mut msaa: ResMut<Msaa>) {
    if msaa.samples != 4 {
        msaa.samples = 4;
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &Velocity)>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok((player, mut transform, velocity)) = query.get_single_mut() {
        transform.translation += Vec3 {
            x: velocity.vel.x * player.speed * time.delta_seconds(),
            y: velocity.vel.y * player.speed * time.delta_seconds(),
            ..default()
        };

        if let Ok(mut cam_transform) = cam_query.get_single_mut() {
            cam_transform.translation = cam_transform.translation.lerp(transform.translation, 0.05);
        }
    }
}

fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &Transform, &mut Velocity)>,
    tile_query: Query<(&Tile, &Transform), Without<Player>>,
) {
    if let Ok((mut player, ply_transform, mut velocity)) = query.get_single_mut() {
        velocity.accel = Vec2::default();

        if keys.pressed(KeyCode::W) {
            velocity.accel.y = 1000.0;
        }

        if keys.pressed(KeyCode::S) {
            velocity.accel.y = -1000.0;
        }

        if keys.pressed(KeyCode::A) {
            velocity.accel.x = -1000.0;
        }

        if keys.pressed(KeyCode::D) {
            velocity.accel.x = 1000.0;
        }

        velocity.vel = velocity
            .vel
            .lerp(velocity.vel + velocity.accel, time.delta_seconds())
            .lerp(Vec2::default(), time.delta_seconds() * velocity.friction)
            .clamp_length(0.0, player.max_speed);

        for (tile, transform) in tile_query.iter() {
            if tile.tile_type != TileType::Clip {
                continue;
            }

            if let Some(collision) = collide_aabb::collide(
                ply_transform.translation
                    + Vec3 {
                        x: 2.0,
                        y: -12.0,
                        ..default()
                    },
                Vec2 { x: 24.0, y: 8.0 },
                transform.translation,
                Vec2 { x: 32.0, y: 32.0 },
            ) {
                match collision {
                    Collision::Left => velocity.vel.x = -4.0,
                    Collision::Right => velocity.vel.x = 4.0,
                    Collision::Top => velocity.vel.y = 4.0,
                    Collision::Bottom => velocity.vel.y = -4.0,
                    Collision::Inside => velocity.vel.y = -32.0,
                }
            }
        }
    }
}

fn main() {
    let mut inv = Inventory::random();

    inv.sort(SortingField::Name, SortingDirection::Asc);
    inv.inspect();

    inv.sort(SortingField::Name, SortingDirection::Desc);
    inv.inspect();

    inv.sort(SortingField::Weight, SortingDirection::Asc);
    inv.inspect();

    inv.sort(SortingField::Weight, SortingDirection::Desc);
    inv.inspect();

    inv.sort(SortingField::Price, SortingDirection::Asc);
    inv.inspect();

    inv.sort(SortingField::Price, SortingDirection::Desc);
    inv.inspect();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(set_msaa)
        .add_system(move_player)
        .add_system(update)
        .run();
}
