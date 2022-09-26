use crate::traits::equipable::Equipable;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Equipment {
    slot: u32,
    damage_mod: f32,
    defense_mod: f32,
    speed_mod: f32,
    slots_mod: f32,
    weight_mod: f32,
}

impl Equipable for Equipment {
    fn can_equip(&self, slot: u32) -> bool {
        self.slot == slot
    }

    fn get_slot(&self) -> u32 {
        self.slot
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
