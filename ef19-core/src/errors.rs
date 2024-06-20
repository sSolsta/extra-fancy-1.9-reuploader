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
    Io(IoError),
    Base64(base64::DecodeError),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingObjectHeader => write!(f, "no object header in level string"),
            Self::Gmd(e) => write!(f, "gmd encode/decode error: {e}"),
            Self::Key(e) => write!(f, "{e}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Base64(e) => write!(f, "base64 decode error: {e}"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingObjectHeader => None,
            Self::Gmd(e) => Some(e),
            Self::Key(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Base64(e) => Some(e),
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
impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}
impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Self::Base64(e)
    }
}
impl From<ZipError> for Error {
    fn from(e: ZipError) -> Self {
        match e {
            ZipError::Io(e) => Self::Io(e),
            ZipError::Base64(e) => Self::Base64(e),
        }
    }
}