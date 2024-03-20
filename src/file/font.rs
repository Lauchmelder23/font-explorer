use log::{debug, error};

use crate::file::{error::{FontError, Result}, loader::FontLoader, outlines::{self, Outlines}, table::{header::FontHeader, hheader::HorizontalHeader, mapping::CharacterMap, maxp::MaximumProfile}};

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,
    outlines: Outlines
}

impl OpenTypeFont {
    pub fn load(filepath: &str) -> Result<OpenTypeFont> {
        let mut loader = FontLoader::from_file(filepath)?;

        let log_and_none = |err: FontError| {
            error!("{}", err);
            None
        };

        // Parse font header first (head)
        let header: FontHeader                  = loader.load_table("head")?;
        let hheader: HorizontalHeader           = loader.load_table("hhea")?;
        let mapping: CharacterMap               = loader.load_table("cmap")?;
        let profile: Option<MaximumProfile>     = loader.load_table("maxp").map_or_else(log_and_none, |result| Some(result));

        debug!("Done loading OpenType tables.");

        let outlines = Outlines::load(&mut loader)?;

        Ok(OpenTypeFont {
            file: String::from(filepath),
            outlines
        })
    }
}

