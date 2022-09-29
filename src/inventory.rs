use crate::equipment::Equipment;
use crate::item::Item;
use crate::traits::*;
use bevy::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Component)]
pub struct Inventory {
    items: Vec<Item>,
    equipment: Vec<Equipment>,
    max_items: u32,
    max_equipment: u32,
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
        while i <= high && cmp(&a[i as usize], &a[((high + low) / 2) as usize]) == Ordering::Less {
            i += 1;
        }

        j -= 1;
        while j >= 0 && cmp(&a[j as usize], &a[((high + low) / 2) as usize]) == Ordering::Greater {
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

    pub fn add_item(&mut self, item: Item) -> bool {
        if !item.valid()
            || self.items.len() as u32 >= self.max_items
            || self.total_weight() + item.weight > self.max_weight
        {
            return false;
        }

        self.items.push(item);

        true
    }

    pub fn take_item(&mut self, item_id: u32) -> bool {
        if let Some((index, _item)) = self.find_item(item_id) {
            self.items.remove(index);

            return true;
        }

        false
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

    pub fn total_weight(&self) -> f32 {
        self.items.iter().map(|e| e.get_weight()).sum()
    }

    pub fn total_price(&self) -> f32 {
        self.items.iter().map(|e| e.get_price()).sum()
    }

    pub fn total_items(&self) -> usize {
        self.items.len()
    }

    pub fn find_item(&self, id: u32) -> Option<(usize, &Item)> {
        for (i, item) in self.items.iter().enumerate() {
            if item.id == id {
                return Some((i, item));
            }
        }

        None
    }

    pub fn find_item_mut(&mut self, id: u32) -> Option<(usize, &mut Item)> {
        for (i, mut item) in self.items.iter_mut().enumerate() {
            if item.id == id {
                return Some((i, item));
            }
        }

        None
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
            "Inventory(items: {} ({} max), equipment: {} ({} max), weight: {:.2}kg ({:.2}kg max))",
            self.items.len(),
            self.max_items,
            self.equipment.len(),
            self.max_equipment,
            self.total_weight(),
            self.max_weight * (1.0 + self.total_weight_mod()),
        )
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![],
            equipment: vec![],
            max_items: 10,
            max_equipment: 3,
            max_weight: 10.0,
        }
    }
}
