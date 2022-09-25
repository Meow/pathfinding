mod equipment;
mod inventory;
mod item;
mod map;
mod player;
mod room;
mod tile;
mod traits;
mod vector2;

use crate::vector2::Vector2;
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
