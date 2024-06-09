use std::collections::HashMap;
use crate::models::object::LevelObject;

struct ObjectList {
    header: HashMap<String, String>,
    objects: Vec<LevelObject>,
}

enum ObjectCodec {
    Encoded(String),
    Decoded(ObjectList),
}

pub enum Song {
    Official(u32),
    Custom(u32),
}

pub struct Level {
    name: String,
    description: String,
    objects: ObjectCodec,
    song: Song,
    version: u32,
    length: u32,
    is_two_player: bool,
    object_count: u32,
    has_low_detail: bool,
}

