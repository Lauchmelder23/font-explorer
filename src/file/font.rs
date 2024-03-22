use log::{debug, warn};

use crate::file::{error::{FontError, Result}, loader::FontLoader, outlines::{self, OutlineLoadConfig, Outlines}, table::{CharacterMap, FontHeader, HorizontalHeader, MaximumProfile}};

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
            warn!("{}", err);
            None
        };
        // Parse font header first (head)
        let header: FontHeader                  = loader.load_table("head", ())?;
        let hheader: HorizontalHeader           = loader.load_table("hhea", ())?;
        let mapping: CharacterMap               = loader.load_table("cmap", ())?;
        let profile: Option<MaximumProfile>     = loader.load_table("maxp", ()).map_or_else(log_and_none, |result| Some(result));

        debug!("Done loading OpenType tables.");

        let config = OutlineLoadConfig {
            head: &header,
            maxp: profile.as_ref()
        };

        let outlines = Outlines::load(&mut loader, config)?;

        Ok(OpenTypeFont {
            file: String::from(filepath),
            outlines
        })
    }
}

