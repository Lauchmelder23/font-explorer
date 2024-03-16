use super::{error::Result, loader::FontLoader, table::{header::FontHeader, mapping::CharacterMap}};

#[derive(Debug)]
pub struct OpenTypeFont {
    file: String,

}

impl OpenTypeFont {
    pub fn load(filepath: &str) -> Result<OpenTypeFont> {
        let mut loader = FontLoader::from_file(filepath)?;

        // Parse font header first (head)
        let header: FontHeader = loader.load_table("head")?;
        // let horizontal_header: HorizontalHeader = loader.load_table("horizontal_header")?;

        let mapping: CharacterMap = loader.load_table("cmap")?;

        Ok(OpenTypeFont {
            file: String::from(filepath)
        })
    }
}

