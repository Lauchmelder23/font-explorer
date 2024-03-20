use std::io::{Read, Seek};

use log::{debug, error};
use serde::Deserialize;

use crate::file::{deserialize_from, error::{Result, FontError}, loader::TableDirectoryEntry};

use super::table::Table;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct MaxpV05 {
    num_glyphs: u16
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct MaxpV10 {
    num_glyphs:             u16,
    points:                 u16,
    contours:               u16,
    composite_points:       u16,
    composite_contours:     u16,
    zones:                  u16,
    twilight_points:        u16,
    storage:                u16,
    function_defs:          u16,
    instruction_defs:       u16,
    stack_elements:         u16,
    size_of_instructions:   u16,
    component_elements:     u16,
    component_depth:        u16
}

#[derive(Debug, Clone, Copy)]
pub enum MaximumProfile {
    Version05(MaxpV05),
    Version10(MaxpV10)
}

impl Table for MaximumProfile {
    fn get_table_name() -> &'static str {
        "Maximum Profile"
    }

    fn load_impl<S>(entry: TableDirectoryEntry, stream: &mut S) -> Result<Self>
        where S: Read + Seek
    {
        let version: u32 = deserialize_from(stream)?;

        let table = match version {
            0x00005000 => Self::Version05(deserialize_from(stream)?),
            0x00010000 => Self::Version10(deserialize_from(stream)?),
            _ => return Err(
                FontError::FontFormatError(
                    Some(entry.offset),
                    format!("maximum profile has unknown table version: {:#08x}", version)
                )
            )
        };
        
        debug!("{:?}", table);
        Ok(table)
    }
}
