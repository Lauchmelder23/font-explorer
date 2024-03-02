use std::io::{Read, Seek};

use bincode::deserialize;
use log::debug;
use serde::Deserialize;

use crate::file::{self, error::Result, file::TableDirectoryEntry};

#[derive(Debug, Deserialize, Copy, Clone, Default)]
struct EncodingRecord {
    platform_id: u16,
    encoding_id: u16,
    subtable_offset: u32
}

#[derive(Debug, Deserialize, Clone)]
pub struct CmapHeader {
    version: u16,
    num_tables: u16,

    #[serde(skip)]
    encoding_records: Vec<EncodingRecord>
}

impl CmapHeader {
    pub fn load<T>(dict_entry: TableDirectoryEntry, stream: &mut T) -> Result<CmapHeader> 
        where T: Read + Seek
    {
        debug!("loading character map header at 0x{:08x}", dict_entry.offset);
        stream.seek(std::io::SeekFrom::Start(dict_entry.offset as u64))?;

        let mut header: CmapHeader = file::deserialize_from(stream)?;

        header.encoding_records = vec![];
        for _ in 0..header.num_tables {
            header.encoding_records.push(file::deserialize_from::<EncodingRecord, _>(stream)?);
        }

        debug!("{:?}", header);

        Ok(header)
    }
}
