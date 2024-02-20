use std::{borrow::{Borrow, BorrowMut}, collections::HashMap, fs::File, io::{Read, Seek}};
use bincode::Options;
use serde::Deserialize;

use super::error::{FontError, Result};

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,

}

impl OpenTypeFont {
    pub fn load(filepath: &str) -> Result<OpenTypeFont> {
        let mut file = File::open(filepath)?;

        let table_dir = TableDirectory::load(&mut file)?;
        dbg!(table_dir);

        Ok(OpenTypeFont {
            file: String::from(filepath)
        })
    }

    pub fn deserialize_from<T, R>(reader: &mut R) -> bincode::Result<T>
        where R: Read,
              T: serde::de::DeserializeOwned
    {
        bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .deserialize_from(reader.by_ref())
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
struct TableDirectoryEntry {
    tag: u32,
    checksum: u32,
    offset: u32,
    length: u32
}

impl TableDirectory {
    fn load<T>(stream: &mut T) -> Result<TableDirectory> 
        where T: Read + Seek 
    {
        let mut table_dir: TableDirectory = OpenTypeFont::deserialize_from(stream)?;
        println!("{:?}", table_dir);

        for _ in 0..table_dir.num_tables {
            let table: TableDirectoryEntry = OpenTypeFont::deserialize_from(stream)?;

            if let Some(_) = table_dir.tables.insert(table.tag, table) {
                return Err(FontError::new((stream.stream_position()? as u32) - 16, "duplicate tag"))
            };
        }

        Ok(table_dir)
    }
}
