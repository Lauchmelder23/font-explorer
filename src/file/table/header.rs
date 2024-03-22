use std::io::{Read, Seek};

use log::{debug, warn};
use serde::Deserialize;

use crate::file::{self, error::Result, loader::TableDirectoryEntry};

use super::table::Table;

#[derive(Debug, Copy, Clone, Deserialize)]
struct BoundingBox {
    left: i16,
    bottom: i16,
    right: i16,
    top: i16
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct FontHeader {
    major_version:          u16,
    minor_version:          u16,
    font_revision:          (u16, u16),
    checksum_adjust:        u32,
    magic_number:           u32,
    flags:                  u16,
    units_per_em:           u16,
    created:                i64,
    modified:               i64,
    max_bbox:               BoundingBox,
    mac_style:              u16,
    lowest_rec_pprem:       u16,
    font_direction:         i16,
    index_to_loc_format:    i16,
    glyph_data_format:      i16,
}

impl Table for FontHeader {
    type UserArgsType = ();

    fn get_table_name() -> &'static str {
        "Font Header"
    }

    fn load_impl<S>(dict_entry: TableDirectoryEntry, stream: &mut S, user_data: Self::UserArgsType) -> Result<Self>
        where S: Read + Seek
    {
        let header: FontHeader = file::deserialize_from(stream)?;
        debug!("{:?}", header);

        if header.magic_number != 0x5F0F3CF5 {
            warn!("wrong magic number: {:08x}", header.magic_number);
        }

        Ok(header)
    }
}
