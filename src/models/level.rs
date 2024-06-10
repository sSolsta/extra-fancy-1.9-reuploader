use std::collections::HashMap;
use crate::models::object::LevelObject;

enum ObjectList {
    Encoded(String),
    Decoded {
        header: HashMap<String, String>,
        objects: Vec<LevelObject>,
    },
}

pub enum Song {
    Official(u32),
    Custom(u32),
}

pub struct Level {
    name: String,
    description: String,
    objects: ObjectList,
    song: Song,
    version: u32,
    length: u32,
    is_two_player: bool,
    object_count: u32,
    has_low_detail: bool,
}

