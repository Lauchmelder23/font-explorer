use std::io::{Read, Seek};

use log::debug;

use crate::file::{error::Result, loader::FontLoader};

#[derive(Debug, Clone, Copy)]
pub struct TrueType {
    
}

impl TrueType {
    pub fn load<S>(loader: &mut FontLoader<S>) -> Result<TrueType>
        where S: Read + Seek
    {
        debug!("Loading TrueType outlines");

        Ok(TrueType {})
    }
}
