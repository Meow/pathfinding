use crate::equipment::Equipment;
use crate::item::Item;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Inventory {
    items: Vec<Item>,
    equipment: [Equipment; 3],
}

impl Inventory {
    pub fn sort(&mut self) {}
}
