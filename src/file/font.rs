use log::error;

use super::{error::{FontError, Result}, loader::FontLoader, table::{header::FontHeader, hheader::HorizontalHeader, mapping::CharacterMap, maxp::MaximumProfile}};

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,
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

        Ok(OpenTypeFont {
            file: String::from(filepath)
        })
    }
}

