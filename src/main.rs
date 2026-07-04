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

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    Playing,
    Restarting,
}

#[derive(Component)]
struct PathfindingNode;

#[derive(Message)]
struct PathfindingEvent;

#[derive(Default, Resource)]
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

/// Which side of the target box was collided with. Reimplements the
/// `bevy::sprite::collide_aabb::Collision` enum removed in Bevy 0.13.
#[derive(Debug, PartialEq, Eq)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

/// Axis-aligned bounding box collision, reimplementing the behavior of the
/// old `bevy::sprite::collide_aabb::collide`. Returns which side of `b` that
/// `a` collided with, or `None` if they do not overlap.
fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Collision::Left, b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Collision::Right, a_min.x - b_max.x)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Collision::Bottom, b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Collision::Top, a_min.y - b_max.y)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        if y_depth.abs() < x_depth.abs() {
            Some(y_collision)
        } else {
            Some(x_collision)
        }
    } else {
        None
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut game_data = GameData::default();

    commands.spawn((
        Camera2d,
        Transform {
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
    ));

    let map = Map::random();

    for room in &map.rooms {
        for tile in &room.tiles {
            let transform = Transform::from_xyz(
                room.pos.x * 32.0 + tile.pos.x * 32.0,
                room.pos.y * 32.0 + tile.pos.y * 32.0,
                0.,
            );

            commands.spawn((
                Sprite::from_image(asset_server.load(&tile.texture_path)),
                transform,
                tile.clone(),
            ));

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
        commands.spawn((
            Sprite::from_image(asset_server.load(&tile.texture_path)),
            Transform::from_xyz(tile.pos.x * 32.0, tile.pos.y * 32.0, 1.0),
            tile.clone(),
        ));
    }

    commands.spawn((
        Sprite::from_image(asset_server.load("player_new_32x32.png")),
        Transform {
            translation: Vec3 {
                x: game_data.spawn.translation.x,
                y: game_data.spawn.translation.y + 8.0,
                z: 1.0,
            },
            ..default()
        },
        Player::default(),
        Inventory::default(),
        Velocity::default(),
    ));

    commands.insert_resource(game_data);
    commands.insert_resource(map);
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &Velocity)>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok((player, mut transform, velocity)) = query.single_mut() {
        transform.translation += Vec3 {
            x: velocity.vel.x * player.speed * time.delta_secs(),
            y: velocity.vel.y * player.speed * time.delta_secs(),
            ..default()
        };

        if let Ok(mut cam_transform) = cam_query.single_mut() {
            cam_transform.translation = cam_transform
                .translation
                .lerp(transform.translation, time.delta_secs() * 4.0);
            cam_transform.translation.z = 2.0;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut commands: Commands,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<(&Player, &mut Transform, &mut Velocity, &mut Inventory)>,
    mut ev_pathfind: MessageWriter<PathfindingEvent>,
    tile_query: Query<(Entity, &Tile, &Transform), Without<Player>>,
) {
    if let Ok((player, mut ply_transform, mut velocity, mut inventory)) = query.single_mut() {
        if !game_data.pathfinding_navigating && keys.just_pressed(KeyCode::F1) {
            ev_pathfind.write(PathfindingEvent);
        }

        if !game_data.pathfinding_navigating && keys.just_pressed(KeyCode::F2) {
            if !game_data.pathfinding_shown {
                ev_pathfind.write(PathfindingEvent);
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

        if keys.just_pressed(KeyCode::F11) {
            println!("Generating a random demo inventory and demonstrating sorting");

            let mut inv = Inventory::random();

            inv.inspect();

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
        }

        let max_speed = if keys.pressed(KeyCode::ShiftLeft) {
            player.max_speed * 2.0
        } else if keys.pressed(KeyCode::ControlLeft) {
            player.max_speed * 0.5
        } else {
            player.max_speed
        } * (1.0 + inventory.total_speed_mod());

        velocity.accel = Vec2::default();

        if !game_data.pathfinding_navigating {
            if keys.pressed(KeyCode::KeyW) {
                velocity.accel.y = 2000.0;
            }

            if keys.pressed(KeyCode::KeyS) {
                velocity.accel.y = -2000.0;
            }

            if keys.pressed(KeyCode::KeyA) {
                velocity.accel.x = -2000.0;
            }

            if keys.pressed(KeyCode::KeyD) {
                velocity.accel.x = 2000.0;
            }

            velocity.vel = velocity
                .vel
                .lerp(velocity.vel + velocity.accel, time.delta_secs())
                .lerp(Vec2::default(), time.delta_secs() * velocity.friction)
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

            if let Some(collision) = collide(
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
                    if *state.get() != AppState::Restarting {
                        next_state.set(AppState::Restarting);
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
    mut ev_pathfind: MessageReader<PathfindingEvent>,
    player_query: Query<&Transform, With<Player>>,
    tile_query: Query<(&Tile, &Transform), Without<Player>>,
    path_node_query: Query<Entity, With<PathfindingNode>>,
) {
    if let Ok(transform) = player_query.single() {
        for _ev in ev_pathfind.read() {
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
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgba(0.75, 0.25, 0.25, 0.25),
                            Vec2::new(32.0, 32.0),
                        ),
                        Transform::from_xyz(pos.0 as f32, pos.1 as f32, 0.5),
                        PathfindingNode,
                    ));
                }

                game_data.nav_nodes = result;
            } else {
                println!("path not found!");
            }
        }
    }
}

fn restart(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    // Only despawn the entities the game spawned (sprites, camera, UI), and
    // only root entities: `despawn` is recursive, so UI children (which have a
    // `ChildOf`) are cleaned up via their parent. This avoids touching the
    // resource-backed and other internal entities that a bare `Query<Entity>`
    // would match in Bevy 0.19.
    query: Query<
        Entity,
        (
            Without<ChildOf>,
            Or<(With<Sprite>, With<Camera>, With<Node>)>,
        ),
    >,
) {
    for ent in &query {
        commands.entity(ent).despawn();
    }

    if *state.get() != AppState::Playing {
        next_state.set(AppState::Playing);
    }
}

fn spawn_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Oswald-SemiBold.ttf");

    let line = |text: &str| {
        (
            Text::new(text),
            TextFont {
                font: font.clone().into(),
                font_size: FontSize::Px(28.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
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
            BackgroundColor(Color::srgba(0., 0., 0., 0.25)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CONTROLS"),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::srgb(0.45, 0.35, 1.0)),
            ));

            parent.spawn(line("WASD: Move"));
            parent.spawn(line("LShift: Sprint"));
            parent.spawn(line("LCtrl: Walk"));
            parent.spawn(line("F1: Toggle Pathfinding"));
            parent.spawn(line("F2: Auto-Navigate to Exit"));
            parent.spawn(line("F3: Equip Items (if possible)"));
            parent.spawn(line("F4: Print Inventory"));
            parent.spawn(line("F5: Sort Inventory (Name, Ascending)"));
            parent.spawn(line("F6: Sort Inventory (Name, Descending)"));
            parent.spawn(line("F7: Sort Inventory (Weight, Ascending)"));
            parent.spawn(line("F8: Sort Inventory (Weight, Descending)"));
            parent.spawn(line("F9: Sort Inventory (Price, Ascending)"));
            parent.spawn(line("F10: Sort Inventory (Price, Descending)"));
            parent.spawn(line("F11: Generate Random Inventory and Sort"));

            parent.spawn((
                Text::new("PLEASE TAKE NOTE OF CONSOLE OUTPUT"),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
            ));
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_message::<PathfindingEvent>()
        .insert_state(AppState::Playing)
        .add_systems(OnEnter(AppState::Playing), (setup, spawn_gui))
        .add_systems(
            Update,
            (move_player, update, pathfind).run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnEnter(AppState::Restarting), restart)
        .run();
}
