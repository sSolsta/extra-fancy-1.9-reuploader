use std::collections::HashMap;
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
    colour: Option<Color>,  // 19 old, 22 new
    z_layer: Option<i8>,  // 24
    z_order: Option<i32>,  // 25
    base_hsv: Option<String>,  // 41 enabled, 43 string
    // other properties
    other_data: HashMap<String, String>,
}

