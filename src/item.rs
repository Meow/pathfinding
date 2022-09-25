use crate::traits::inventorizable::Inventorizable;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Item {
    name: String,
    desc: String,
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

    fn get_price(&self) -> f32 {
        self.price
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}
