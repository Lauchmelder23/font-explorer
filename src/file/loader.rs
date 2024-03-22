use std::{collections::HashMap, fs::File, io::{BufReader, Read, Seek}};
use log::{debug, info};
use serde::Deserialize;

use crate::file::{self, table::Table, error::{Result, FontError}};

macro_rules! tag_to_str {
    ($tag: expr) => ($tag.to_be_bytes().iter().map(|&byte| char::from(byte)).collect::<String>());
}

macro_rules! tag_to_int {
    ($tag: expr) => ($tag.bytes().fold(0u32, |left, right| (left << 8) | right as u32));
}

#[derive(Debug)]
pub struct FontLoader<S> where
    S: Read + Seek
{
    table_dir: TableDirectory,
    stream: S
}
impl FontLoader<BufReader<File>> {
    pub fn from_file(filepath: &str) -> Result<Self> {
        info!("loading font from file '{}'", filepath);

        let file = File::open(filepath)?;
        let mut stream = BufReader::new(file);

        Self::new(stream)
    }
}

impl<S> FontLoader<S>
    where S: Read + Seek
{
    pub fn new(mut stream: S) -> Result<Self> {
        let mut table_dir = TableDirectory::load(&mut stream)?;

        Ok(FontLoader {
            table_dir,
            stream
        })
    }

    pub fn load_table<T>(&mut self, tag: &str) -> Result<T>
    where T: Table
    {
        let tag_id = tag_to_int!(tag);
        let entry = self.table_dir.tables.remove(&tag_id)
            .ok_or_else(move || FontError::FontFormatError(None, format!("Missing table 0x{:08}", tag_id)))?;

        T::load(entry, &mut self.stream)
    }

    pub fn get_table_dir(&self) -> &TableDirectory {
        &self.table_dir
    }

    pub fn check_tables_present<'a, I>(&self, tables: I) -> Option<String>
        where I: Iterator<Item = &'a&'a str>
    {
        let missing_tags = tables
            .filter(|&tag| !self.table_dir.tables.contains_key(&tag_to_int!(tag)))
            .fold(String::new(), |left, &right| { 
                if !left.is_empty() {
                    format!(", {}", right)
                } else {
                    String::from(right)
                }
            });

        if !missing_tags.is_empty() {
            return Some(missing_tags)
        }

        None
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct TableDirectory {
    pub sfnt_version: u32,
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
    pub fn load<S>(stream: &mut S) -> Result<TableDirectory> 
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

    pub fn get_tables(&self) -> &HashMap<u32, TableDirectoryEntry> {
        &self.tables
    }
}
