use std::io::{Read, Seek};

use log::{debug, warn};
use serde::Deserialize;

use crate::file::{self, error::Result, loader::TableDirectoryEntry, BoundingBox};

use super::table::Table;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct FontHeader {
    pub major_version:          u16,
    pub minor_version:          u16,
    pub font_revision:          (u16, u16),
    pub checksum_adjust:        u32,
    pub magic_number:           u32,
    pub flags:                  u16,
    pub units_per_em:           u16,
    pub created:                i64,
    pub modified:               i64,
    pub max_bbox:               BoundingBox,
    pub mac_style:              u16,
    pub lowest_rec_pprem:       u16,
    pub font_direction:         i16,
    pub index_to_loc_format:    i16,
    pub glyph_data_format:      i16,
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
