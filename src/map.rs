use crate::room::Room;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Map {
    rooms: Vec<Room>,
}
