use crate::item::Item;
use crate::traits::*;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Equipment {
    pub item_id: String,
    pub slot: EquipmentSlot,
    pub damage_mod: f32,
    pub defense_mod: f32,
    pub speed_mod: f32,
    pub weight_mod: f32,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Equipment(slot: {:?}, item_id: {}, damage: {}, defense: {}, speed: {}, weight: {})",
            self.slot,
            self.item_id,
            self.damage_mod,
            self.defense_mod,
            self.speed_mod,
            self.weight_mod
        )
    }
}

impl From<Item> for Equipment {
    fn from(item: Item) -> Self {
        Self {
            slot: match item.id {
                1 => EquipmentSlot::Weapon,
                2 => EquipmentSlot::Armor,
                3..=5 => EquipmentSlot::Booster,
                _ => EquipmentSlot::None,
            },
            item_id: item.prefab_id,
            damage_mod: match item.id {
                1 => 1.0,
                _ => 0.0,
            },
            defense_mod: match item.id {
                2 => 1.0,
                _ => 0.0,
            },
            speed_mod: match item.id {
                3..=4 => 0.2,
                _ => 0.0,
            },
            weight_mod: match item.id {
                3 => 0.5,
                4 => -0.2,
                _ => 0.0,
            },
        }
    }
}

impl Equipable for Equipment {
    fn get_slot(&self) -> EquipmentSlot {
        self.slot
    }

    fn get_item_id(&self) -> &String {
        &self.item_id
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
            slot: EquipmentSlot::None,
            item_id: "".to_string(),
            damage_mod: 0.0,
            defense_mod: 0.0,
            speed_mod: 0.0,
            weight_mod: 0.0,
        }
    }
}
