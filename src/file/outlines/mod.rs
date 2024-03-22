use std::io::{Read, Seek};

use log::debug;

pub use self::truetype::TrueType;

use super::{error::{FontError, Result}, loader::FontLoader, table::{FontHeader, MaximumProfile}};

mod truetype;

pub struct OutlineLoadConfig<'a> {
    pub head: &'a FontHeader,
    pub maxp: Option<&'a MaximumProfile>
}

#[derive(Debug, Clone, Copy)]
pub enum Outlines {
    TrueType(TrueType)
}

impl Outlines {
    pub fn load<S>(loader: &mut FontLoader<S>, config: OutlineLoadConfig) -> Result<Outlines>
        where S: Read + Seek 
    {
        debug!("Loading font outlines");

        let sfnt_version = loader.get_table_dir().sfnt_version;
        Ok(match sfnt_version {
            0x00010000 => {
                if config.maxp.is_none() {
                    return Err(
                        FontError::FontFormatError(None, String::from("Maxp table is needed to decode TrueType outlines"))
                    )
                }

                Outlines::TrueType(TrueType::load(loader, config.head, config.maxp.unwrap())?)
            },
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
