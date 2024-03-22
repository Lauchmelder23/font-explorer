use std::io::{Read, Seek};

use crate::file::{deserialize_from, error::{FontError, Result}, loader::TableDirectoryEntry, table::Table};

pub type Locations = Vec<u32>;

impl Table for Locations {
    type UserArgsType = (i16, u16);

    fn get_table_name() -> &'static str {
        "loca"
    }

    fn load_impl<S>(entry: TableDirectoryEntry, stream: &mut S, (format, num_glyphs): Self::UserArgsType) -> Result<Self>
        where S: Read + Seek
    {

        match format {
            0 => {
                let mut locations: Vec<u16> = Vec::with_capacity(num_glyphs as usize + 1);
                locations = deserialize_from(stream)?;

                let locations = locations.iter().map(|&offset| 2*(offset as u32)).collect::<Vec<u32>>();
                Ok(locations)
            },
            1 => {
                let mut locations: Vec<u32> = Vec::with_capacity(num_glyphs as usize + 1);
                locations = deserialize_from(stream)?;

                Ok(locations)
            },

            _ => Err(FontError::FontFormatError(Some(entry.offset), format!("Unknown location offset format: {}", format)))
        }
    }
}
