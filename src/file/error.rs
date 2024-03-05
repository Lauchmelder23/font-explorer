use core::fmt;
use std::{error, result};

#[derive(Debug)]
pub enum FontError {
    FontFormatError(Option<u32>, String),
    IOError(std::io::Error),
    DeserializeError(bincode::Error),
}

pub type Result<T> = result::Result<T, FontError>;

impl FontError {
    pub fn new(offset: Option<u32>, msg: &str) -> Self {
        FontError::FontFormatError(offset, msg.into())
    }
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontError::FontFormatError(offset, msg) => {
                write!(f, "Error in font file")?;
                if let Some(value) = offset {
                    write!(f, " (at 0x{:08x})", value)?;
                }
                write!(f, ": {}", msg)
            },
            FontError::IOError(_) => write!(f, "IO error while reading font file"),
            FontError::DeserializeError(_) => write!(f, "Deserialisation error while reading font file")
        }
    }
}

impl error::Error for FontError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            FontError::FontFormatError(_, _) => None,
            FontError::IOError(err) => Some(err),
            FontError::DeserializeError(err) => Some(err)
        }
    }
}

impl From<std::io::Error> for FontError {
    fn from(value: std::io::Error) -> Self {
        FontError::IOError(value)
    }
}

impl From<bincode::Error> for FontError {
    fn from(value: bincode::Error) -> Self {
        FontError::DeserializeError(value)
    }
}
