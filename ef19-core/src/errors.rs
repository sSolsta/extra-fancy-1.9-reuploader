use std::fmt;
use std::io::Error as IoError;
use crate::codec::gdshare::GmdError;
use crate::codec::ZipError;

#[derive(Debug)]
pub enum KeyError {
    Missing { key: String },
    Invalid { key: String, val: String },
}
impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Missing { key } => write!(f, "missing key {key}"),
            Self::Invalid { key, val } =>write!(f, "invalid value {val} for key {key}"),
        }
    }
}
impl std::error::Error for KeyError {}

#[derive(Debug)]
pub enum Error {
    MissingObjectHeader,
    Gmd(GmdError),
    Key(KeyError),
    Zip(ZipError),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingObjectHeader => write!(f, "no object header in level string"),
            Self::Gmd(e) => write!(f, "gmd encode/decode error: {e}"),
            Self::Key(e) => write!(f, "{e}"),
            Self::Zip(e) => write!(f, "gzip error: {e}"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingObjectHeader => None,
            Self::Gmd(e) => Some(e),
            Self::Key(e) => Some(e),
            Self::Zip(e) => Some(e),
        }
    }
}
impl From<GmdError> for Error {
    fn from(e: GmdError) -> Self {
        Self::Gmd(e)
    }
}
impl From<KeyError> for Error {
    fn from(e: KeyError) -> Self {
        Self::Key(e)
    }
}
impl From<ZipError> for Error {
    fn from(e: ZipError) -> Self {
        Self::Zip(e)
    }
}