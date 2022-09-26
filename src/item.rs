use crate::traits::inventorizable::Inventorizable;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Item {
    name: String,
    desc: String,
    texture_path: String,
    price: f32,
    weight: f32,
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
