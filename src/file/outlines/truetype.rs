use std::io::{Read, Seek};

use log::debug;

use crate::file::{error::{FontError, Result}, loader::FontLoader};

#[derive(Debug, Clone, Copy)]
pub struct TrueType {
    
}

impl TrueType {
    const REQUIRED_TAGS: [&'static str; 2] = ["glyf", "loca"];

    pub fn load<S>(loader: &mut FontLoader<S>) -> Result<TrueType>
        where S: Read + Seek
    {
        debug!("Loading TrueType outlines");

        if let Some(missing_tags) = loader.check_tables_present(TrueType::REQUIRED_TAGS.iter()) {
            return Err(FontError::FontFormatError(None, format!("The following tables are required, but were missing from the table directory: {}", missing_tags)));
        }

        Ok(TrueType {})
    }
}
