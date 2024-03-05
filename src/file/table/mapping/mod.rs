mod delta_mapper;

use std::{collections::HashMap, io::{Read, Seek}};

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
    pub fn load<S>(dict_entry: TableDirectoryEntry, stream: &mut S) -> Result<CmapHeader> 
        where S: Read + Seek
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

// TODO: Really I shouldn't parse every code point into a hash map...
// rather i sohuld return a handle to the file segment and perform a binary
// search as inteded
type CharacterMap = HashMap<char, u16>;

pub fn load_character_map<S>(dict_entry: TableDirectoryEntry, stream: &mut S) -> Result<CharacterMap>
    where S: Read + Seek
{
    debug!("loading character map at 0x{:08x}", dict_entry.offset);

    let table_offset = dict_entry.offset as u64;
    let header = CmapHeader::load(dict_entry, stream)?;

    // For now only Unicode 2.0 BMP is supported
    let Some(result) = 
        header.encoding_records.iter()
        .find(|item| item.platform_id == 0 && item.encoding_id == 3) 
    else {
        todo!();
    };

    stream.seek(std::io::SeekFrom::Start(table_offset + result.subtable_offset as u64))?;
    match file::deserialize_from::<u16, _>(stream)? {
        4 => delta_mapper::load(stream),
        _ => todo!()
    }
}
