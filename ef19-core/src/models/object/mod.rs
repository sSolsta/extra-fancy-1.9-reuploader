use std::collections::HashMap;
use super::macros::attr_from_map;
pub use color::Color;

mod variants;
mod color;

pub struct LevelObject {
    // properties we want fast access to
    id: u16,  // 1
    x_pos: f32,  // 2
    y_pos: f32,  // 3
    flip_x: bool,  // 4
    flip_y: bool,  // 5
    rotation: f32,  // 6
    color: Option<Color>,  // 19 old, 22 new
    z_layer: Option<i8>,  // 24
    z_order: Option<i32>,  // 25
    base_hsv: Option<String>,  // 41 enabled, 43 string
    // other properties
    other_data: HashMap<String, String>,
}

impl LevelObject {
    pub fn from_str(string: &str) -> Option<LevelObject> {
        
        None
    }
    // consumes map
    pub fn from_map(mut map: HashMap<String, String>) -> Option<LevelObject> {
        // required properties
        let id = attr_from_map!(map, "1", u16);
        let x_pos = attr_from_map!(map, "2", f32);
        let y_pos = attr_from_map!(map, "3", f32);
        // defaulted properties
        let flip_x = attr_from_map!(map, "4", bool, default=false);
        let flip_y = attr_from_map!(map, "5", bool, default=false);
        let rotation = attr_from_map!(map, "6", f32, default=0.);
        // optional properties
        let z_layer = attr_from_map!(map, "24", Option<i8>);
        let z_order = attr_from_map!(map, "25", Option<i32>);
        let base_hsv = attr_from_map!(map, "25", Option<String>);
        // colour (special case)
        let color = match attr_from_map!(map, "19", Option<u8>) {
            None | Some(0) => match attr_from_map!(map, "22", Option<u16>) {
                Some(v) => Color::from_new_id(v),
                None => None,
            }
            Some(v) => Color::from_old_id(v),
        };
        Some(LevelObject {
            id,
            x_pos,
            y_pos,
            flip_x,
            flip_y,
            rotation,
            color,
            z_layer,
            z_order,
            base_hsv,
            other_data: map,
        })
    }
}

