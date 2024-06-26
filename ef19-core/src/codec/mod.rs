use std::{
    collections::HashMap,
    error::Error,
    io::Read,
    io::Error as IoError,
    fmt,
};
use itertools::Itertools;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use flate2::{
    read::{GzDecoder, GzEncoder},
    Compression,
};

pub mod gdshare;
pub mod server;
pub mod format;

pub fn escaped_string(raw: &[u8]) -> String {
    let mut string = String::new();
    for c in raw {
        match *c {
            9 => string.push_str(r"\t"),
            10 => string.push_str(r"\r"),
            13 => string.push_str(r"\n"),
            32..=91 | 93..=126 => string.push(*c as char),
            92 => string.push_str(r"\\"),
            _ => string.push_str(&format!("\\x{:02x}", c)),
        }
    }
    string
}

pub fn escaped_string_quotes(raw: &[u8]) -> String {
    let mut string = String::new();
    for c in raw {
        match *c {
            9 => string.push_str(r"\t"),
            10 => string.push_str(r"\r"),
            13 => string.push_str(r"\n"),
            32..=33 | 35..=91 | 93..=126 => string.push(*c as char),
            34 => string.push_str(r#"\""#),
            92 => string.push_str(r"\\"),
            _ => string.push_str(&format!("\\x{:02x}", c)),
        }
    }
    string
}

// deserialise k:v:k:v style string
pub fn deserialise_kv(input: &str, sep: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (k, v) in input.split(sep).tuples() {
        map.insert(
            k.to_string(),
            v.to_string(),
        );
    }
    map
}

// serialise k:v:k:v style string
pub fn serialise_kv(map: &HashMap<String, String>, sep: &str) -> String {
    let mut serialised = String::new();
        let mut kvs = map.iter();
        if let Some((k, v)) = kvs.next() {
            serialised.push_str(k);
            serialised.push_str(sep);
            serialised.push_str(v);
            
            for (k, v) in kvs {
                serialised.push_str(sep);
                serialised.push_str(k);
                serialised.push_str(sep);
                serialised.push_str(v);
            }
        }
        serialised
}

// error enum for gzip functions
#[derive(Debug)]
pub enum ZipError {
    Base64(base64::DecodeError),
    Io(IoError),
}
impl fmt::Display for ZipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Base64(e) => write!(f, "base64 decode error: {e}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}
impl std::error::Error for ZipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Base64(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}
impl From<base64::DecodeError> for ZipError {
    fn from(e: base64::DecodeError) -> Self {
        Self::Base64(e)
    }
}
impl From<IoError> for ZipError {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}

// gzip encode
pub fn zip_string(unzipped: &str) -> Result<String, ZipError> {
    let mut encoder = GzEncoder::new(unzipped.as_bytes(), Compression::new(9));
    let mut bytes = Vec::new();
    
    encoder.read_to_end(&mut bytes)?;
    
    Ok(URL_SAFE.encode(bytes))
}

// gzip decode
pub fn unzip_string(zipped: &str) -> Result<String, ZipError> {
    let bytes = URL_SAFE.decode(zipped)?;
    
    let mut unzipped = String::new();
    let mut decoder = GzDecoder::new(bytes.as_slice());
    
    decoder.read_to_string(&mut unzipped)?;
    
    Ok(unzipped)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("1".to_string(), "2".to_string());
        map.insert("3".to_string(), "4".to_string());
        map.insert("5".to_string(), "6".to_string());
        map.insert("8".to_string(), "shit".to_string());
        
        let serialised = serialise_kv(&map, ":");
        println!("{}", serialised);
        
        // there's no guarantee that the map will output in a specific order,
        // so we have to split and iterate to check
        for (k, v) in serialised.split(":").tuples() {
            if k == "1" { assert_eq!(v, "2"); }
            else if k == "3" { assert_eq!(v, "4"); }
            else if k == "5" { assert_eq!(v, "6"); }
            else if k == "8" { assert_eq!(v, "shit"); }
            else { panic!(); }
        }
    }
    #[test]
    fn deserialise() {
        let object = "1:2:3:4:5:6:8:shit";
        let map = deserialise_kv(object, ":");
        assert_eq!(map.get("1").unwrap(), "2");
        assert_eq!(map.get("3").unwrap(), "4");
        assert_eq!(map.get("5").unwrap(), "6");
        assert_eq!(map.get("8").unwrap(), "shit");
    }
    #[test]
    fn zip_cycle() {
        let string = "awawawawawawawawawawawawawawawawa".to_string();
        let zipped = zip_string(&string).unwrap();
        println!("{}", zipped);
        let unzipped = unzip_string(&zipped).unwrap();
        assert_eq!(string, unzipped);
    }
}


