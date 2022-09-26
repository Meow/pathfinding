pub trait Equipable {
    fn can_equip(&self, slot: u32) -> bool;
    fn get_slot(&self) -> u32;
    fn get_damage_mod(&self) -> f32;
    fn get_defense_mod(&self) -> f32;
    fn get_speed_mod(&self) -> f32;
    fn get_weight_mod(&self) -> f32;
}
