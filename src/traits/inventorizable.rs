pub trait Inventorizable {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_texture_path(&self) -> String;
    fn get_price(&self) -> f32;
    fn get_weight(&self) -> f32;
}
