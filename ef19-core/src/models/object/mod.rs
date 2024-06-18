use std::collections::HashMap;
use super::macros::attr_from_map;
pub use color::Color;
use crate::codec::format::GdFormat;

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
    // consumes map
    pub fn from_map(mut map: HashMap<String, String>) -> Option<LevelObject> {
        // required properties
        let id = attr_from_map!(map, "1", u16);
        if id == 0 { return None; }
        let x_pos = attr_from_map!(map, "2", f32);
        let y_pos = attr_from_map!(map, "3", f32);
        // defaulted properties
        let flip_x = attr_from_map!(map, "4", bool, default=false);
        let flip_y = attr_from_map!(map, "5", bool, default=false);
        let rotation = attr_from_map!(map, "6", f32, default=0.);
        // optional properties
        let z_layer = attr_from_map!(map, "24", Option<i8>);
        let z_order = attr_from_map!(map, "25", Option<i32>);
        // base_hsv (special case)
        let base_hsv = if attr_from_map!(map, "41", bool, default=false) {
            attr_from_map!(map, "43", Option<String>)
        } else { None };
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
    
    pub fn into_inner(mut self) -> HashMap<String, String> {
        self.other_data.insert("1".to_string(), self.id.gd_format());
        self.other_data.insert("2".to_string(), self.x_pos.gd_format());
        self.other_data.insert("3".to_string(), self.y_pos.gd_format());
        self.other_data.insert("4".to_string(), self.flip_x.gd_format());
        self.other_data.insert("5".to_string(), self.flip_y.gd_format());
        self.other_data.insert("6".to_string(), self.rotation.gd_format());
        if let Some(v) = self.z_layer {
            self.other_data.insert("24".to_string(), v.gd_format());
        }
        if let Some(v) = self.z_order {
            self.other_data.insert("25".to_string(), v.gd_format());
        }
        if let Some(v) = self.base_hsv {
            self.other_data.insert("41".to_string(), true.gd_format());
            self.other_data.insert("43".to_string(), v.gd_format());
        }
        if let Some(v) = self.color {
            self.other_data.insert("19".to_string(), v.gd_format());
        }
        
        self.other_data
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn map_cycle_minimal() {
        let mut map = HashMap::new();
        map.insert("1".to_string(), "68".to_string());
        map.insert("2".to_string(), "20".to_string());
        map.insert("3".to_string(), "44.3".to_string());
        
        let obj = LevelObject::from_map(map).unwrap();
        
        assert_eq!(obj.id, 68);
        assert_eq!(obj.x_pos, 20.0);
        assert_eq!(obj.y_pos, 44.3);
        assert_eq!(obj.flip_x, false);
        assert_eq!(obj.flip_y, false);
        assert_eq!(obj.rotation, 0.);
        
        assert_eq!(obj.z_layer, None);
        assert_eq!(obj.z_order, None);
        assert_eq!(obj.base_hsv, None);
        assert_eq!(obj.color, None);
        
        let map = obj.into_inner();
        for (k, v) in map.iter() {
            match k.as_str() {
                "1" => { assert_eq!(v, "68"); },
                "2" => { assert_eq!(v, "20"); },
                "3" => { assert_eq!(v, "44.3"); },
                "4" => { assert_eq!(v, "0"); },
                "5" => { assert_eq!(v, "0"); },
                "6" => { assert_eq!(v, "0"); },
                i => { panic!("Unexpected key {}", i); },
            }
        }
    }
    
    #[test]
    fn map_cycle_maximal() {
        let mut map = HashMap::new();
        map.insert("1".to_string(), "68".to_string());
        map.insert("2".to_string(), "20.22".to_string());
        map.insert("3".to_string(), "19".to_string());
        map.insert("4".to_string(), "0".to_string());
        map.insert("5".to_string(), "1".to_string());
        map.insert("6".to_string(), "22.545".to_string());
        
        map.insert("24".to_string(), "-1".to_string());
        map.insert("25".to_string(), "-8".to_string());
        
        map.insert("41".to_string(), "1".to_string());
        // not valid hsv string it's just a placeholder
        map.insert("43".to_string(), "auawauawuawa".to_string());
        // oldstyle colour takes precedence over newstyle
        map.insert("19".to_string(), "3".to_string());
        map.insert("22".to_string(), "3".to_string());
        
        let obj = LevelObject::from_map(map).unwrap();
        
        assert_eq!(obj.id, 68);
        assert_eq!(obj.x_pos, 20.22);
        assert_eq!(obj.y_pos, 19.0);
        assert_eq!(obj.flip_x, false);
        assert_eq!(obj.flip_y, true);
        assert_eq!(obj.rotation, 22.545);
        
        assert_eq!(obj.z_layer, Some(-1));
        assert_eq!(obj.z_order, Some(-8));
        assert_eq!(obj.base_hsv.clone().unwrap(), "auawauawuawa");
        assert_eq!(obj.color, Some(Color::Col1));
        
        let map = obj.into_inner();
        for (k, v) in map.iter() {
            match k.as_str() {
                "1" => { assert_eq!(v, "68"); },
                "2" => { assert_eq!(v, "20.22"); },
                "3" => { assert_eq!(v, "19"); },
                "4" => { assert_eq!(v, "0"); },
                "5" => { assert_eq!(v, "1"); },
                "6" => { assert_eq!(v, "22.545"); },
                "24" => { assert_eq!(v, "-1"); },
                "25" => { assert_eq!(v, "-8"); },
                "41" => { assert_eq!(v, "1"); },
                "43" => { assert_eq!(v, "auawauawuawa"); },
                "19" => { assert_eq!(v, "3"); },
                "22" => (),
                i => { panic!("Unexpected key {}", i); },
            }
        }
    }
    
    #[test]
    fn invalid_object() {
        let mut map = HashMap::new();
        map.insert("1".to_string(), "1".to_string());
        map.insert("2".to_string(), "0".to_string());
        // missing y_pos - invalid
        map.insert("4".to_string(), "0".to_string());
        map.insert("5".to_string(), "0".to_string());
        map.insert("6".to_string(), "0".to_string());
        
        let obj = LevelObject::from_map(map);
        assert!(obj.is_none());
    }
    
    #[test]
    fn newstyle_color() {
        let mut map = HashMap::new();
        map.insert("1".to_string(), "1".to_string());
        map.insert("2".to_string(), "0".to_string());
        map.insert("3".to_string(), "0".to_string());
        map.insert("22".to_string(), "3".to_string());
        
        let obj = LevelObject::from_map(map).unwrap();
        assert_eq!(obj.color, Some(Color::Col3));
        
        let mut map = HashMap::new();
        map.insert("1".to_string(), "1".to_string());
        map.insert("2".to_string(), "0".to_string());
        map.insert("3".to_string(), "0".to_string());
        map.insert("22".to_string(), "1003".to_string());
        
        let obj = LevelObject::from_map(map).unwrap();
        assert_eq!(obj.color, Some(Color::DLine));
    }
}

