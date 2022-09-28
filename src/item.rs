use crate::traits::inventorizable::Inventorizable;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub texture_path: String,
    pub price: f32,
    pub weight: f32,
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

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item(name: {}, price: {}, weight: {})",
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
            price: 0.0,
            weight: 0.0,
        }
    }
}
