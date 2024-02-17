use std::io::{Read, Seek};

use crate::file::typed_reader::TypedReader;

use super::TableReader;

#[derive(Debug, Copy, Clone)]
pub enum FontFlags {
    BaselineAtZero              = (1 << 0),
    LeftSidebearingAtZero       = (1 << 1),
    InstrMayDependOnPointSize   = (1 << 2),
    InstrMayAlterAdvanceWidth   = (1 << 3),
    FontIsLossless              = (1 << 11),
    FontConverted               = (1 << 12),
    FontOptimizedForClearType   = (1 << 13),
    LastResort                  = (1 << 14)
}

#[derive(Debug, Copy, Clone)]
pub enum MacStyles {
    Bold        = (1 << 0),
    Italic      = (1 << 1),
    Underline   = (1 << 2),
    Outline     = (1 << 3),
    Shadow      = (1 << 4),
    Condensed   = (1 << 5),
    Extended    = (1 << 6)
}

#[derive(Debug)]
pub struct BoundingBox {
    left: i16,
    bottom: i16,
    right: i16,
    top: i16
}

#[derive(Debug)]
pub struct FontHeader {
    major_version:          u16,
    minor_version:          u16,
    font_revision:          (u16, u16),
    checksum_adjust:        u32,
    magic:                  u32,
    flags:                  u16,
    units_per_em:           u16,
    created:                i64,
    modified:               i64,
    max_bbox:               BoundingBox,
    mac_style:              u16,
    lowest_rec_ppem:        u16,
    font_direction:         i16,
    index_to_loc_format:    i16,
    glyph_data_format:      i16
}

impl TableReader for FontHeader {
    const TAG: u32 = 0x68656164;

    fn read<T>(record: &super::TableRecord, buf: &mut T) -> std::io::Result<Self>
            where Self: Sized,
                  T: Read + Seek 
    {
        let mut data = vec![0u8; 54];
        buf.read_exact(&mut data)?;

        let mut reader = TypedReader::from(data);

        Ok(FontHeader {
            major_version:          reader.read_u16()?,
            minor_version:          reader.read_u16()?,
            font_revision:          (reader.read_u16()?, reader.read_u16()?),
            checksum_adjust:        reader.read_u32()?,
            magic:                  reader.read_u32()?,
            flags:                  reader.read_u16()?,
            units_per_em:           reader.read_u16()?,
            created:                reader.read_i64()?,
            modified:               reader.read_i64()?,
            max_bbox:
                BoundingBox {
                    left:           reader.read_i16()?,
                    bottom:         reader.read_i16()?,
                    right:          reader.read_i16()?,
                    top:            reader.read_i16()?
            },
            mac_style:              reader.read_u16()?,
            lowest_rec_ppem:        reader.read_u16()?,
            font_direction:         reader.read_i16()?,
            index_to_loc_format:    reader.read_i16()?,
            glyph_data_format:      reader.read_i16()?
        })
    }
}
