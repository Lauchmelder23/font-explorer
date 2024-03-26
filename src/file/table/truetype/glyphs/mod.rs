use std::{collections::HashMap, io::{Read, Seek}};

use serde::Deserialize;

use crate::file::{deserialize_from, error::Result, loader::TableDirectoryEntry, table::Table, BoundingBox};

mod simple;
pub use simple::SimpleGlyph;

#[derive(Debug, Clone, Copy)]
pub struct CompositeGlyph {
    
}

#[derive(Debug, Clone)]
pub enum GlyphDescription {
    Simple(SimpleGlyph),
    Composite(CompositeGlyph)
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct GlyphHeader {
    num_contours: i16,
    bounding_box: BoundingBox
}

#[derive(Debug, Clone)]
pub struct Glyph {
    header: GlyphHeader,
    description: GlyphDescription
}

pub type Glyphs = HashMap<u32, Glyph>;

impl Table for Glyphs {
    type UserArgsType = u16;
    
    fn get_table_name() -> &'static str {
        "glyf"
    }

    fn load_impl<S>(entry: TableDirectoryEntry, stream: &mut S, num_glyphs: Self::UserArgsType) -> Result<Self>
        where S: Read + Seek
    {
        for _ in 0..num_glyphs {
            let offset = stream.stream_position()? - entry.offset as u64;
            let header: GlyphHeader = deserialize_from(stream)?;

            let description = GlyphDescription::load(&header, stream)?;
        }

        todo!()
    }
}

impl GlyphDescription {
    fn load<S>(header: &GlyphHeader, stream: &mut S) -> Result<Self>
        where S: Read + Seek
    {
        Ok(match header.num_contours {
            -1 => todo!(),
            _  => GlyphDescription::Simple(SimpleGlyph::load(header, stream)?)
        })
    }
}
