use crate::equipment::Equipment;
use crate::traits::*;
use bevy::prelude::*;
use rand::Rng;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub texture_path: String,
    pub prefab_id: String,
    pub price: f32,
    pub weight: f32,
    pub id: u32,
}

impl Item {
    pub fn random_prefab() -> Self {
        let n = rand::thread_rng().gen::<f32>();

        if n > 0.99 {
            Self::prefab("garry")
        } else if n > 0.9 {
            Self::prefab("companion")
        } else if n > 0.7 {
            Self::prefab("ally")
        } else if n > 0.35 {
            Self::prefab("armor")
        } else {
            Self::prefab("weapon")
        }
    }
}

impl From<Equipment> for Item {
    fn from(eq: Equipment) -> Self {
        Self::prefab(&eq.item_id)
    }
}

impl Prefabricatable for Item {
    fn prefab(id: &str) -> Self {
        match id {
            "weapon" => Item {
                id: 1,
                name: "The Weapon".to_string(),
                desc: "It seems great for dealing with hostile aliens who like to hop on people's heads.".to_string(),
                texture_path: "weapon_32x32.png".to_string(),
                prefab_id: "weapon".to_string(),
                weight: 2.0,
                price: 100.0,
            },
            "armor" => Item {
                id: 2,
                name: "The Armor".to_string(),
                desc: "Injects user with morphine. Doesn't come with a helmet.".to_string(),
                texture_path: "armor_32x32.png".to_string(),
                prefab_id: "armor".to_string(),
                weight: 4.0,
                price: 250.0,
            },
            "companion" => Item {
                id: 3,
                name: "The Companion".to_string(),
                desc: "...will never threaten to stab you and, in fact, cannot speak.".to_string(),
                texture_path: "cube_32x32.png".to_string(),
                prefab_id: "companion".to_string(),
                weight: 5.0,
                price: 999.0,
            },
            "ally" => Item {
                id: 4,
                name: "The Ally".to_string(),
                desc: "Speaks in an accent that is beyond some people's range of hearing. Probably won't betray you.".to_string(),
                texture_path: "item_32x32.png".to_string(),
                prefab_id: "ally".to_string(),
                weight: 2.5,
                price: 0.0,
            },
            "garry" => Item {
                id: 5,
                name: "The New Man".to_string(),
                desc: "Makes mods, hates penguins, likes colors purple and black.".to_string(),
                texture_path: "missing_texture_32x32.png".to_string(),
                prefab_id: "garry".to_string(),
                weight: 999.0,
                price: -1000000.0,
            },
            _ => Item::default(),
        }
    }
}

impl Inventorizable for Item {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.desc.clone()
    }

    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }

    fn get_price(&self) -> f32 {
        self.price
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}

impl Validatable for Item {
    fn valid(&self) -> bool {
        self.id > 0
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item(name: '{}', price: ${:.2}, weight: {:.2}kg)",
            self.name, self.price, self.weight
        )
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: "Default Item".to_string(),
            desc: "This item has no description.".to_string(),
            texture_path: "missing_texture_16x16.png".to_string(),
            prefab_id: "default".to_string(),
            price: 0.0,
            weight: 0.0,
            id: 0,
        }
    }
}
