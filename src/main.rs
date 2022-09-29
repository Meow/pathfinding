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
    inventory::Inventory, inventory::SortingDirection, inventory::SortingField, item::Item,
    map::Map, player::Player, tile::Tile, tile::TileType, velocity::Velocity,
};
use bevy::prelude::*;
use bevy::sprite::{collide_aabb, collide_aabb::Collision};
use pathfinding::prelude::astar;

#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance_squared(&self, p2: &Pos) -> u32 {
        (self.0 - p2.0).pow(2) as u32 + (self.1 - p2.1).pow(2) as u32
    }

    fn from_transform(trans: &Transform) -> Self {
        Pos(trans.translation.x as i32, trans.translation.y as i32)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Playing,
    Restarting,
}

#[derive(Component)]
struct PathfindingNode;

struct PathfindingEvent;

#[derive(Default)]
struct GameData {
    portal_blue: Transform,
    portal_orange: Transform,
    spawn: Transform,
    exit: Transform,
    pathfinding_shown: bool,
    pathfinding_navigating: bool,
    last_nav_node: usize,
    next_nav_node: usize,
    nav_nodes: Vec<Pos>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut game_data = GameData::default();

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

    for room in &map.rooms {
        for tile in &room.tiles {
            let transform = Transform::from_xyz(
                room.pos.x * 32.0 + tile.pos.x * 32.0,
                room.pos.y * 32.0 + tile.pos.y * 32.0,
                0.,
            );

            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&tile.texture_path),
                    transform,
                    ..default()
                })
                .insert(tile.clone());

            match tile.tile_type {
                TileType::Spawn => game_data.spawn = transform,
                TileType::Exit => game_data.exit = transform,
                TileType::PortalA => game_data.portal_blue = transform,
                TileType::PortalB => game_data.portal_orange = transform,
                _ => (),
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
            transform: Transform {
                translation: Vec3 {
                    x: game_data.spawn.translation.x,
                    y: game_data.spawn.translation.y + 8.0,
                    z: 1.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Player::default())
        .insert(Inventory::default())
        .insert(Velocity::default());

    commands.insert_resource(game_data);
    commands.insert_resource(map);
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
            cam_transform.translation = cam_transform
                .translation
                .lerp(transform.translation, time.delta_seconds() * 4.0);
            cam_transform.translation.z = 2.0;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&Player, &mut Transform, &mut Velocity, &mut Inventory)>,
    mut ev_pathfind: EventWriter<PathfindingEvent>,
    tile_query: Query<(Entity, &Tile, &Transform), Without<Player>>,
) {
    if let Ok((player, mut ply_transform, mut velocity, mut inventory)) = query.get_single_mut() {
        if !game_data.pathfinding_navigating && keys.just_pressed(KeyCode::F1) {
            ev_pathfind.send(PathfindingEvent);
        }

        if !game_data.pathfinding_navigating && keys.just_pressed(KeyCode::F2) {
            if !game_data.pathfinding_shown {
                ev_pathfind.send(PathfindingEvent);
            }

            game_data.pathfinding_navigating = true;
        }

        if keys.just_pressed(KeyCode::F3) {
            inventory.equip_items();
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F4) {
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F5) {
            println!("Sorting inventory by name, ascending order");
            inventory.sort(SortingField::Name, SortingDirection::Asc);
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F6) {
            println!("Sorting inventory by name, descending order");
            inventory.sort(SortingField::Name, SortingDirection::Desc);
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F7) {
            println!("Sorting inventory by weight, ascending order");
            inventory.sort(SortingField::Weight, SortingDirection::Asc);
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F8) {
            println!("Sorting inventory by weight, descending order");
            inventory.sort(SortingField::Weight, SortingDirection::Desc);
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F9) {
            println!("Sorting inventory by price, ascending order");
            inventory.sort(SortingField::Price, SortingDirection::Asc);
            inventory.inspect();
        }

        if keys.just_pressed(KeyCode::F10) {
            println!("Sorting inventory by price, descending order");
            inventory.sort(SortingField::Price, SortingDirection::Desc);
            inventory.inspect();
        }

        let max_speed = if keys.pressed(KeyCode::LShift) {
            player.max_speed * 2.0
        } else if keys.pressed(KeyCode::LControl) {
            player.max_speed * 0.5
        } else {
            player.max_speed
        } * (1.0 + inventory.total_speed_mod());

        velocity.accel = Vec2::default();

        if !game_data.pathfinding_navigating {
            if keys.pressed(KeyCode::W) {
                velocity.accel.y = 2000.0;
            }

            if keys.pressed(KeyCode::S) {
                velocity.accel.y = -2000.0;
            }

            if keys.pressed(KeyCode::A) {
                velocity.accel.x = -2000.0;
            }

            if keys.pressed(KeyCode::D) {
                velocity.accel.x = 2000.0;
            }

            velocity.vel = velocity
                .vel
                .lerp(velocity.vel + velocity.accel, time.delta_seconds())
                .lerp(Vec2::default(), time.delta_seconds() * velocity.friction)
                .clamp_length(0.0, max_speed);
        } else if !game_data.nav_nodes.is_empty() {
            let target_pos = &game_data.nav_nodes[game_data.next_nav_node];
            let current_pos = Pos::from_transform(&ply_transform);
            let x_diff = current_pos.0 - target_pos.0;
            let y_diff = current_pos.1 - (target_pos.1 + 8);

            velocity.vel.x = if x_diff.abs() < 1 {
                0.0
            } else if x_diff > 0 {
                -96.0
            } else {
                96.0
            };

            velocity.vel.y = if y_diff.abs() < 1 {
                0.0
            } else if y_diff > 0 {
                -96.0
            } else {
                96.0
            };

            if current_pos.distance_squared(target_pos) < 128
                && game_data.next_nav_node < game_data.nav_nodes.len() - 1
            {
                game_data.last_nav_node = game_data.next_nav_node;
                game_data.next_nav_node += 1;
            }
        }

        let mut teleport = 0u32;

        for (ent, tile, transform) in tile_query.iter() {
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
                    if inventory.add_item(Item::random_prefab()) {
                        commands.entity(ent).despawn();

                        inventory.inspect();
                    }
                } else if tile.tile_type == TileType::Exit {
                    if state.current() != &AppState::Restarting {
                        state.set(AppState::Restarting).unwrap();
                    }
                } else if tile.tile_type == TileType::PortalA {
                    teleport = 2;
                } else if tile.tile_type == TileType::PortalB {
                    teleport = 1;
                } else if !game_data.pathfinding_navigating {
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
                ply_transform.translation = game_data.portal_blue.translation
                    + Vec3 {
                        x: 32.0,
                        z: 1.0,
                        ..default()
                    }
            }
            2 => {
                ply_transform.translation = game_data.portal_orange.translation
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

fn pathfind(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut ev_pathfind: EventReader<PathfindingEvent>,
    player_query: Query<&Transform, With<Player>>,
    tile_query: Query<(&Tile, &Transform), Without<Player>>,
    path_node_query: Query<Entity, With<PathfindingNode>>,
) {
    if let Ok(transform) = player_query.get_single() {
        for _ev in ev_pathfind.iter() {
            game_data.pathfinding_shown = !game_data.pathfinding_shown;

            println!("showing pathfinding: {}", game_data.pathfinding_shown);

            if !game_data.pathfinding_shown {
                for ent in path_node_query.iter() {
                    commands.entity(ent).despawn();
                }

                game_data.nav_nodes = vec![];

                return;
            }

            let goal = Pos::from_transform(&game_data.exit);

            if let Some((result, moves)) = astar(
                &Pos::from_transform(transform),
                |p| {
                    let mut successors: Vec<Pos> = vec![];

                    for (tile, tile_transform) in tile_query.iter() {
                        if tile.tile_type != TileType::Clip {
                            let trans = Pos::from_transform(tile_transform);
                            let dist = p.distance_squared(&trans);

                            if dist > 128 && dist < 1100 {
                                successors.push(trans);
                            }
                        }
                    }

                    successors
                        .into_iter()
                        .map(|p| (p, 1))
                        .collect::<Vec<(Pos, u32)>>()
                },
                |p| p.distance_squared(&goal) / 32,
                |p| p.distance_squared(&goal) < 128,
            ) {
                println!("  path is {} tiles long", moves);

                for pos in result.iter() {
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.75, 0.25, 0.25, 0.25),
                                custom_size: Some(Vec2::new(32.0, 32.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(pos.0 as f32, pos.1 as f32, 0.5),
                            ..default()
                        })
                        .insert(PathfindingNode);
                }

                game_data.nav_nodes = result;
            } else {
                println!("path not found!");
            }
        }
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

fn spawn_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let default_style = TextStyle {
        font: asset_server.load("fonts/Oswald-SemiBold.ttf"),
        font_size: 28.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(64.0)),
                margin: UiRect {
                    left: Val::Auto,
                    ..default()
                },
                ..default()
            },
            color: Color::rgba(0., 0., 0., 0.25).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "CONTROLS",
                TextStyle {
                    font: asset_server.load("fonts/Oswald-SemiBold.ttf"),
                    font_size: 48.0,
                    color: Color::rgb(0.45, 0.35, 1.0),
                },
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "WASD: Move",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "LShift: Sprint",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "LCtrl: Walk",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F1: Toggle Pathfinding",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F2: Auto-Navigate to Exit",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F3: Equip Items (if possible)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F4: Print Inventory",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F5: Sort Inventory (Name, Ascending)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F6: Sort Inventory (Name, Descending)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F7: Sort Inventory (Weight, Ascending)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F8: Sort Inventory (Weight, Descending)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F9: Sort Inventory (Price, Ascending)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "F10: Sort Inventory (Price, Descending)",
                default_style.clone(),
            ));

            parent.spawn_bundle(TextBundle::from_section(
                "PLEASE TAKE NOTE OF CONSOLE OUTPUT",
                TextStyle {
                    font: asset_server.load("fonts/Oswald-SemiBold.ttf"),
                    font_size: 28.0,
                    color: Color::rgb(0.9, 0.2, 0.2),
                },
            ));
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<PathfindingEvent>()
        .add_state(AppState::Playing)
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(setup)
                .with_system(spawn_gui),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(move_player)
                .with_system(update)
                .with_system(pathfind),
        )
        .add_system_set(SystemSet::on_enter(AppState::Restarting).with_system(restart))
        .add_startup_system(set_msaa)
        .run();
}
