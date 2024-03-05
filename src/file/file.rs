use std::{collections::HashMap, fs::File, io::{BufReader, Read, Seek}};
use log::{debug, info};
use serde::Deserialize;

use crate::file::{self, error::FontError, table::{mapping, CmapHeader}};

use super::{error::Result, table::FontHeader};

macro_rules! tag_to_str {
    ($tag: expr) => ($tag.to_be_bytes().iter().map(|&byte| char::from(byte)).collect::<String>());
}

macro_rules! tag_to_int {
    ($tag: expr) => ($tag.bytes().fold(0u32, |left, right| (left << 8) | right as u32));
}

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,

}

impl OpenTypeFont {
    const REQUIRED_TAGS: [&'static str; 8] = ["cmap", "head", "hhea", "hmtx", "maxp", "name", "OS/2", "post"];

    pub fn load(filepath: &str) -> Result<OpenTypeFont> {
        info!("loading font from file '{}'", filepath);

        let file = File::open(filepath)?;
        let mut stream = BufReader::new(file);
    
        let mut table_dir = TableDirectory::load(&mut stream)?;

        let missing_tags = OpenTypeFont::REQUIRED_TAGS.iter()
            .filter(|&tag| !table_dir.tables.contains_key(&tag_to_int!(tag)))
            .fold(String::new(), |left, &right| { 
                if !left.is_empty() {
                    format!(", {}", right)
                } else {
                    String::from(right)
                }
            });

        if !missing_tags.is_empty() {
            return Err(FontError::FontFormatError(None, format!("The following tables are required, but were missing from the table directory: {}", missing_tags)));
        }

        let error_func = |tag: u32| { move || FontError::FontFormatError(None, format!("Missing table: 0x{:08}", tag)) };

        // Parse font header first (head)
        let tag: u32 = tag_to_int!("head");
        let entry = table_dir.tables.remove(&tag).ok_or_else(error_func(tag))?;
        let header = FontHeader::load(entry, &mut stream)?;

        let tag = tag_to_int!("cmap");
        let entry = table_dir.tables.remove(&tag).ok_or_else(error_func(tag))?;
        let mapping = mapping::load_character_map(entry, &mut stream)?;

        Ok(OpenTypeFont {
            file: String::from(filepath)
        })
    }
}


#[derive(Deserialize, Debug, Default)]
struct TableDirectory {
    sfnt_version: u32,
    num_tables: u16,

    search_range: u16,
    entry_selector: u16,
    range_shift: u16,

    #[serde(skip)]
    tables: HashMap<u32, TableDirectoryEntry>
}

#[derive(Deserialize, Debug, Default)]
pub struct TableDirectoryEntry {
    tag: u32,
    pub checksum: u32,
    pub offset: u32,
    length: u32
}

impl TableDirectory {
    fn load<S>(stream: &mut S) -> Result<TableDirectory> 
        where S: Read + Seek 
    {
        debug!("loading font table directory at 0x{:08x}", stream.stream_position()?);

        let mut table_dir: TableDirectory = file::deserialize_from(stream)?;
        info!("{:?}", table_dir);

        for _ in 0..table_dir.num_tables {
            let table: TableDirectoryEntry = file::deserialize_from(stream)?;
            debug!("found table {}", tag_to_str!(table.tag));

            if let Some(_) = table_dir.tables.insert(table.tag, table) {
                return Err(FontError::new(Some((stream.stream_position()? as u32) - 16), "duplicate tag"))
            };
        }

        Ok(table_dir)
    }
}
