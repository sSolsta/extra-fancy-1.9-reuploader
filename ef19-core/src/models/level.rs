use std::collections::HashMap;
use crate::models::object::LevelObject;
use crate::codec;

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

impl Level {
    // it would be better to make an error type but i am super tired and i don't care enough
    pub fn decode_objects(&mut self) -> Option<bool> {
        match &self.objects {
            ObjectList::Decoded{ .. } => Some(false),
            ObjectList::Encoded(string) => {
                let decompressed = codec::unzip_string(&string).ok()?;
                let mut split = decompressed.split_terminator(";")
                    .map(|x| codec::deserialise_kv(x, ","));
                
                let header = split.next()?;
                let objects = split.filter_map(|x| LevelObject::from_map(x)).collect();
                
                self.objects = ObjectList::Decoded{ header, objects };
                Some(true)
            }
        }
    }
    
    /* pub fn encode_objects(&mut self) -> Option<bool> {
        match &self.objects {
            ObjectList::Encoded(_) => Some(false),
            ObjectList::Decoded{ header, objects } => {
                
            }
        }
    } */
    
    // pub fn from_server_string(string: &str) -> Option<Level> {}
    
    // pub fn from_server_map(map: &HashMap<String, String>) -> Option<Level> {}
}

