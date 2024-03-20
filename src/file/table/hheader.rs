use std::io::{Read, Seek};

use log::debug;
use serde::Deserialize;

use crate::file::{deserialize_from, error::Result, loader::TableDirectoryEntry};

use super::table::Table;

#[derive(Debug, Deserialize)]
pub struct MinSideBearing {
    left: i16,
    right: i16
}

#[derive(Debug, Deserialize)]
pub struct Caret {
    rise: i16,
    run: i16,
    offset: i16
}

#[derive(Debug, Deserialize)]
pub struct HorizontalHeader {
    version:                (u16, u16),
    ascender:               i16,
    descender:              i16,
    line_gap:               i16,
    advance_width_max:      u16,
    min_side_bearing:       MinSideBearing,
    x_max_extent:           i16,
    caret:                  Caret,
    _reserved:              (i16, i16, i16, i16),
    metric_data_format:     i16,
    number_of_h_metrics:    u16
}

impl Table for HorizontalHeader {
    fn get_table_name() -> &'static str {
        "Horizontal Header"
    }

    fn load_impl<S>(entry: TableDirectoryEntry, stream: &mut S) -> Result<Self>
            where S: Read + Seek
    {
        let hheader: Self = deserialize_from(stream)?;
        debug!("{:?}", hheader);

        Ok(hheader)
    }
}
