use super::{TableReader, TableRecord};

#[derive(Debug)]
pub struct CharacterGlyphMapping {

}

impl TableReader for CharacterGlyphMapping {
    const TAG: u32 = 0x636d6170;

    fn read<T>(record: &TableRecord, buf: &mut T) -> std::io::Result<Self>
        where Self: Sized,
              T: std::io::prelude::Read 
    {
        Ok(CharacterGlyphMapping {  })
    }
}
