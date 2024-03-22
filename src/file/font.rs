use log::{debug, error};

use crate::file::{error::{FontError, Result}, loader::FontLoader, outlines::{self, Outlines}, table::{FontHeader, HorizontalHeader, CharacterMap, MaximumProfile}};

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,
    outlines: Outlines
}

impl OpenTypeFont {
    const REQUIRED_TAGS: [&'static str; 8] = ["cmap", "head", "hhea", "hmtx", "maxp", "name", "OS/2", "post"];

    pub fn load(filepath: &str) -> Result<OpenTypeFont> {
        let mut loader = FontLoader::from_file(filepath)?;

        if let Some(missing_tags) = loader.check_tables_present(OpenTypeFont::REQUIRED_TAGS.iter()) {
            return Err(FontError::FontFormatError(None, format!("The following tables are required, but were missing from the table directory: {}", missing_tags)));
        }

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

