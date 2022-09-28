use crate::traits::equipable::Equipable;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Equipment {
    slot: u32,
    item_id: u32,
    damage_mod: f32,
    defense_mod: f32,
    speed_mod: f32,
    weight_mod: f32,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Equipment(slot: {}, item_id: {}, damage: {}, defense: {}, speed: {}, weight: {})",
            self.slot,
            self.item_id,
            self.damage_mod,
            self.defense_mod,
            self.speed_mod,
            self.weight_mod
        )
    }
}

impl Equipable for Equipment {
    fn can_equip(&self, slot: u32) -> bool {
        self.slot == slot
    }

    fn get_slot(&self) -> u32 {
        self.slot
    }

    fn get_item_id(&self) -> u32 {
        self.item_id
    }

    fn get_damage_mod(&self) -> f32 {
        self.damage_mod
    }

    fn get_defense_mod(&self) -> f32 {
        self.defense_mod
    }

    fn get_speed_mod(&self) -> f32 {
        self.speed_mod
    }

    fn get_weight_mod(&self) -> f32 {
        self.weight_mod
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            slot: 0,
            item_id: 0,
            damage_mod: 0.0,
            defense_mod: 0.0,
            speed_mod: 0.0,
            weight_mod: 0.0,
        }
    }
}
