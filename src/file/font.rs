use super::{error::Result, loader::FontLoader, table::{header::FontHeader, hheader::HorizontalHeader, mapping::CharacterMap}};

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,

}

impl OpenTypeFont {
    pub fn load(filepath: &str) -> Result<OpenTypeFont> {
        let mut loader = FontLoader::from_file(filepath)?;

        // Parse font header first (head)
        let header: FontHeader          = loader.load_table("head")?;
        let hheader: HorizontalHeader   = loader.load_table("hhea")?;
        let mapping: CharacterMap       = loader.load_table("cmap")?;

        Ok(OpenTypeFont {
            file: String::from(filepath)
        })
    }
}

