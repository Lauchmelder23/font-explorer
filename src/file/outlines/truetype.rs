use std::io::{Read, Seek};

use log::debug;

use crate::file::{error::{FontError, Result}, loader::FontLoader, table::{FontHeader, Glyphs, Locations, MaximumProfile, MaxpV05, Table}};

#[derive(Debug, Clone, Copy)]
pub struct TrueType {
    
}

impl TrueType {
    const REQUIRED_TAGS: [&'static str; 2] = ["glyf", "loca"];

    pub fn load<S>(loader: &mut FontLoader<S>, header: &FontHeader, maxp: &MaximumProfile) -> Result<TrueType>
        where S: Read + Seek
    {
        debug!("Loading TrueType outlines");

        if let Some(missing_tags) = loader.check_tables_present(TrueType::REQUIRED_TAGS.iter()) {
            return Err(FontError::FontFormatError(None, format!("The following tables are required, but were missing from the table directory: {}", missing_tags)));
        }

        let num_glyphs = match maxp {
            MaximumProfile::Version05(table) => table.num_glyphs,
            MaximumProfile::Version10(table) => table.num_glyphs
        };

        let locations: Locations = loader.load_table("loca", (header.index_to_loc_format, num_glyphs))?;
        let glyphs: Glyphs = loader.load_table("glyf", num_glyphs)?;

        Ok(TrueType {})
    }
}
