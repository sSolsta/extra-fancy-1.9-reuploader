use std::fmt;

#[derive(Debug)]
pub enum KeyError {
    Missing { key: String },
    Invalid { key: String, val: String },
}
impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Missing { key } => f.write_str(&format!("missing key {key}")),
            Self::Invalid { key, val } => f.write_str(&format!("invalid value {val} for key {key}")),
        }
    }
}
impl std::error::Error for KeyError {}


/* pub struct KeyError {
    key: String
}
impl KeyError {
    pub fn new(key: &str) -> KeyError {
        KeyError { key: key.to_string() }
    }

impl std::error::Error for KeyError */