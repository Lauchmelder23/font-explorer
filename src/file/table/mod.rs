use std::io::{Read, Seek};

mod cmap;
pub use cmap::CharacterGlyphMapping;

mod head;
pub use head::FontHeader;

#[derive(Debug)]
pub struct TableRecord {
    pub tag: u32,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32
}

pub trait TableReader {
    const TAG: u32;

    fn read<T>(record: &TableRecord, buf: &mut T) -> std::io::Result<Self>
        where Self: Sized,
              T: Read + Seek;
}
