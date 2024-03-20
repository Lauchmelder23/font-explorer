use std::io::{Read, Seek};

use log::debug;

pub use self::truetype::TrueType;

use super::{error::{Result, FontError}, loader::FontLoader};

mod truetype;

#[derive(Debug, Clone, Copy)]
pub enum Outlines {
    TrueType(TrueType)
}

impl Outlines {
    pub fn load<S>(loader: &mut FontLoader<S>) -> Result<Outlines>
        where S: Read + Seek 
    {
        debug!("Loading font outlines");

        let sfnt_version = loader.get_table_dir().sfnt_version;
        Ok(match sfnt_version {
            0x00010000 => Outlines::TrueType(TrueType::load(loader)?),
            0x4F54544F => todo!(),
            _ => {
                return Err(
                    FontError::FontFormatError(
                        Some(0), 
                        format!("Unrecognized sfnt version: {:#08x}. Cannot lood glyph outlines.", sfnt_version)
                    )
                );
            }
        })
    }
}
