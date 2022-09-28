use crate::equipment::Equipment;
use crate::item::Item;
use crate::traits::equipable::Equipable;
use crate::traits::inventorizable::Inventorizable;
use bevy::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Inventory {
    items: Vec<Item>,
    equipment: Vec<Equipment>,
    max_items: u32,
    equipment_slots: u32,
    max_weight: f32,
}

pub enum SortingDirection {
    Asc,
    Desc,
}

pub enum SortingField {
    Name,
    Price,
    Weight,
}

fn partition<T>(a: &mut [T], cmp: &impl Fn(&T, &T) -> Ordering, low: isize, high: isize) -> isize {
    let mut i = low - 1;
    let mut j = high + 1;

    loop {
        i += 1;
        while i < high && cmp(&a[i as usize], &a[((high + low) / 2) as usize]) == Ordering::Less {
            i += 1;
        }

        j -= 1;
        while j > 0 && cmp(&a[j as usize], &a[((high + low) / 2) as usize]) == Ordering::Greater {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        a.swap(i as usize, j as usize);
    }
}

fn quicksort<T>(a: &mut [T], cmp: &impl Fn(&T, &T) -> Ordering, low: isize, high: isize) {
    if low < 0 || high < 0 || low >= high {
        return;
    }

    let p = partition(a, cmp, low, high);

    quicksort(a, cmp, low, p);
    quicksort(a, cmp, p + 1, high);
}

impl Inventory {
    pub fn sort(&mut self, sf: SortingField, sd: SortingDirection) {
        let size = (self.items.len() - 1) as isize;
        let op = match sd {
            SortingDirection::Asc => Ordering::Greater,
            _ => Ordering::Less,
        };

        match sf {
            SortingField::Name => quicksort(
                &mut self.items,
                &|a, b| {
                    let res = a
                        .get_name()
                        .partial_cmp(&b.get_name())
                        .unwrap_or(Ordering::Equal);

                    if res == op {
                        Ordering::Greater
                    } else if res == Ordering::Equal {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                },
                0,
                size,
            ),
            SortingField::Price => quicksort(
                &mut self.items,
                &|a, b| {
                    let res = a
                        .get_price()
                        .partial_cmp(&b.get_price())
                        .unwrap_or(Ordering::Equal);

                    if res == op {
                        Ordering::Greater
                    } else if res == Ordering::Equal {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                },
                0,
                size,
            ),
            SortingField::Weight => quicksort(
                &mut self.items,
                &|a, b| {
                    let res = a
                        .get_weight()
                        .partial_cmp(&b.get_weight())
                        .unwrap_or(Ordering::Equal);

                    if res == op {
                        Ordering::Greater
                    } else if res == Ordering::Equal {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                },
                0,
                size,
            ),
        }
    }

    pub fn total_damage_mod(&self) -> f32 {
        self.equipment.iter().map(|e| e.get_damage_mod()).sum()
    }

    pub fn total_defense_mod(&self) -> f32 {
        self.equipment.iter().map(|e| e.get_defense_mod()).sum()
    }

    pub fn total_speed_mod(&self) -> f32 {
        self.equipment.iter().map(|e| e.get_speed_mod()).sum()
    }

    pub fn total_weight_mod(&self) -> f32 {
        self.equipment.iter().map(|e| e.get_weight_mod()).sum()
    }

    pub fn inspect(&self) {
        println!("{}", self);
        println!("Items:");

        for item in self.items.iter() {
            println!("{}", item);
        }

        println!("Equipment:");

        for equipment in self.equipment.iter() {
            println!("{}", equipment);
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut inv = Inventory::default();

        for n in 0..inv.max_items {
            inv.items.push(Item {
                name: format!("Random item {}", n),
                desc: "This item was randomly generated".to_string(),
                weight: rng.gen::<f32>(),
                price: rng.gen::<f32>() * 100.0,
                ..default()
            });
        }

        inv
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Inventory(items: {}, equipment: {})",
            self.items.len(),
            self.equipment.len()
        )
    }
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
