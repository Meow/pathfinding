use crate::equipment::Equipment;
use crate::item::Item;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Inventory {
    items: Vec<Item>,
    equipment: Vec<Equipment>,
    max_items: u32,
    equipment_slots: u32,
    max_weight: f32,
}

impl Inventory {
    pub fn sort(&mut self) {}
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![],
            equipment: vec![],
            max_items: 10,
            equipment_slots: 3,
            max_weight: 10.0,
        }
    }
}
