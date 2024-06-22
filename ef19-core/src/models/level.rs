use std::collections::HashMap;
use crate::models::object::LevelObject;
use crate::codec;
use crate::errors::{KeyError, Error, EResult};

#[derive(Debug)]
struct ObjectList {
    header: HashMap<String, String>,
    objects: Vec<LevelObject>,
}

impl ObjectList {
    pub fn from_str(object_str: &str) -> EResult<Self> {
        let decompressed = codec::unzip_string(object_str)?;
        let mut split = decompressed.split_terminator(";")
            .map(|x| codec::deserialise_kv(x, ","));
        let header = match split.next() {
            Some(header) => {
                if header.is_empty() {
                    Err(Error::MissingObjectHeader)
                } else {
                    Ok(header)
                }
            }
            None => Err(Error::MissingObjectHeader),
        }?;
        let objects = split.filter_map(|x| LevelObject::from_map(x).ok()).collect();
        Ok(ObjectList {header, objects})
    }
    
    pub fn string(&self) -> EResult<String> {
        let mut object_str = String::new();
        object_str.push_str(&codec::serialise_kv(&self.header, ","));
        object_str.push_str(";");
        for obj in &self.objects {
            let map = obj.map();
            object_str.push_str(&codec::serialise_kv(&map, ","));
            object_str.push_str(";");
        }
        
        Ok(codec::zip_string(&object_str)?)
    }
    
    pub fn into_string(self) -> EResult<String> {
        let mut object_str = String::new();
        object_str.push_str(&codec::serialise_kv(&self.header, ","));
        object_str.push_str(";");
        for obj in self.objects {
            let map = obj.into_map();
            object_str.push_str(&codec::serialise_kv(&map, ","));
            object_str.push_str(";");
        }
        
        Ok(codec::zip_string(&object_str)?)
    }
}

#[derive(Debug)]
pub enum Song {
    Official(u32),
    Custom(u32),
}

#[derive(Debug)]
pub struct Level {
    name: String,
    description: String,
    object_str: String,
    object_list: Option<ObjectList>,
    song: Song,
    version: u32,
    length: u32,
    is_two_player: bool,
    object_count: u32,
    has_low_detail: bool,
}

impl Level {
    
    // pub fn from_server_string(string: &str) -> Option<Level> {}
    
    // pub fn from_server_map(map: &HashMap<String, String>) -> Option<Level> {}
}

