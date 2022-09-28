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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Playing,
    Restarting,
    Inventory,
}

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

    for tile in &map.objects {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(&tile.texture_path),
                transform: Transform::from_xyz(tile.pos.x * 32.0, tile.pos.y * 32.0, 1.0),
                ..default()
            })
            .insert(tile.clone());
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
            cam_transform.translation.z = 2.0;
        }
    }
}

fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    tile_query: Query<(&Tile, &Transform), Without<Player>>,
) {
    if keys.just_pressed(KeyCode::Tab) && state.current() != &AppState::Inventory {
        state.set(AppState::Inventory).unwrap();
    }

    if let Ok((mut player, mut ply_transform, mut velocity)) = query.get_single_mut() {
        let max_speed = if keys.pressed(KeyCode::LShift) {
            player.max_speed * 2.0
        } else if keys.pressed(KeyCode::LControl) {
            player.max_speed * 0.5
        } else {
            player.max_speed
        };

        velocity.accel = Vec2::default();

        if keys.pressed(KeyCode::W) {
            velocity.accel.y = 1500.0;
        }

        if keys.pressed(KeyCode::S) {
            velocity.accel.y = -1500.0;
        }

        if keys.pressed(KeyCode::A) {
            velocity.accel.x = -1500.0;
        }

        if keys.pressed(KeyCode::D) {
            velocity.accel.x = 1500.0;
        }

        velocity.vel = velocity
            .vel
            .lerp(velocity.vel + velocity.accel, time.delta_seconds())
            .lerp(Vec2::default(), time.delta_seconds() * velocity.friction)
            .clamp_length(0.0, max_speed);

        let mut blue_pos = Transform::default();
        let mut orange_pos = Transform::default();
        let mut teleport = 0u32;

        for (tile, transform) in tile_query.iter() {
            if !matches!(
                tile.tile_type,
                TileType::Clip
                    | TileType::Item
                    | TileType::Exit
                    | TileType::PortalA
                    | TileType::PortalB
            ) {
                continue;
            }

            match tile.tile_type {
                TileType::PortalA => blue_pos = *transform,
                TileType::PortalB => orange_pos = *transform,
                _ => (),
            };

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
                if tile.tile_type == TileType::Item {
                } else if tile.tile_type == TileType::Exit {
                    if state.current() != &AppState::Restarting {
                        state.set(AppState::Restarting).unwrap();
                    }
                } else if tile.tile_type == TileType::PortalA {
                    teleport = 2;
                } else if tile.tile_type == TileType::PortalB {
                    teleport = 1;
                } else {
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

        match teleport {
            1 => {
                ply_transform.translation = blue_pos.translation
                    + Vec3 {
                        x: 32.0,
                        z: 1.0,
                        ..default()
                    }
            }
            2 => {
                ply_transform.translation = orange_pos.translation
                    + Vec3 {
                        x: -32.0,
                        z: 1.0,
                        ..default()
                    }
            }
            _ => (),
        };
    }
}

fn restart(mut state: ResMut<State<AppState>>, mut commands: Commands, query: Query<Entity>) {
    for ent in &query {
        commands.entity(ent).despawn();
    }

    if state.current() != &AppState::Playing {
        state.overwrite_replace(AppState::Playing).unwrap();
    }
}

fn spawn_gui(
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(65.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "THEORETICALLY THERE WOULD BE INVENTORY HERE",
                        TextStyle {
                            font: asset_server.load("fonts/Oswald-SemiBold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0., 0., 0.),
                        },
                    ));
                });
        });
}

fn destroy_gui(mut commands: Commands) {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Playing)
        .add_system_set(SystemSet::on_enter(AppState::Playing).with_system(setup))
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(move_player)
                .with_system(update),
        )
        .add_system_set(SystemSet::on_enter(AppState::Restarting).with_system(restart))
        .add_system_set(SystemSet::on_enter(AppState::Inventory).with_system(spawn_gui))
        .add_system_set(SystemSet::on_exit(AppState::Inventory).with_system(destroy_gui))
        .add_startup_system(set_msaa)
        .run();
}
