#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Booster,
    None,
}

pub trait Equipable {
    fn get_slot(&self) -> EquipmentSlot;
    fn get_item_id(&self) -> &String;
    fn get_damage_mod(&self) -> f32;
    fn get_defense_mod(&self) -> f32;
    fn get_speed_mod(&self) -> f32;
    fn get_weight_mod(&self) -> f32;
}
