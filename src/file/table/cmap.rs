use std::{collections::HashMap, io::{Read, Seek, SeekFrom}};
use crate::file::common::new_error;

use super::{TableReader, TableRecord};

#[derive(Debug)]
pub struct CharacterGlyphMapping {
    version: u16,
    encodings: Vec<EncodingRecord>
}

impl CharacterGlyphMapping {
    fn read_subtable<T>(data: &[u8], buf: &mut T, record: &TableRecord) -> std::io::Result<EncodingRecord>
        where T: Seek + Read
    {
        let platform = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let encoding = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let offset = u32::from_be_bytes(data[4..8].try_into().unwrap());

        let next_entry_pos = buf.stream_position()?;
        buf.seek(SeekFrom::Start((record.offset as u64) + (offset as u64)))?;

        let encoding_record = EncodingRecord::new(platform, encoding, buf);

        buf.seek(std::io::SeekFrom::Start(next_entry_pos))?;

        encoding_record
    }
}

impl TableReader for CharacterGlyphMapping {
    const TAG: u32 = 0x636d6170;

    fn read<T>(record: &TableRecord, buf: &mut T) -> std::io::Result<Self>
        where Self: Sized,
              T: Read + Seek
    {
        let mut header_buf = [0u8; 4];
        buf.read_exact(&mut header_buf)?;

        let version = u16::from_be_bytes(header_buf[0..2].try_into().unwrap());
        let num_tables = u16::from_be_bytes(header_buf[2..4].try_into().unwrap());

        let mut records_data: Vec<u8> = vec![0; 8 * (num_tables as usize)];
        buf.read_exact(&mut records_data)?;

        let encodings = (0..num_tables)
            .map(|i| &records_data[8*(i as usize)..8*(i as usize)+8])
            .map(|data| CharacterGlyphMapping::read_subtable(data, buf, record))
            .collect::<std::io::Result<_>>()?;

        Ok(CharacterGlyphMapping {
            version, encodings
        })
    }
}

type GlyphMap = HashMap<char, u16>;

#[derive(Debug)]
pub struct EncodingRecord {
    platform: u16,
    encoding: u16,

    glyphs: GlyphMap
}

impl EncodingRecord {
    fn new<T>(platform: u16, encoding: u16, buf: &mut T) -> std::io::Result<EncodingRecord> 
        where T: Read + Seek 
    {
        let mut header_buf = [0u8; 6];
        buf.read_exact(&mut header_buf)?;

        let format = u16::from_be_bytes(header_buf[0..2].try_into().unwrap());
        let length = u16::from_be_bytes(header_buf[2..4].try_into().unwrap());
        let _ = u16::from_be_bytes(header_buf[4..6].try_into().unwrap());

        Ok(EncodingRecord { 
            platform, encoding,
            glyphs: EncodingRecord::get_glyph_map(format, length - 6, buf)?
        })
    }

    fn get_glyph_map<T>(format: u16, length: u16, buf: &mut T) -> std::io::Result<GlyphMap>
        where T: Read + Seek
    {
        match format {
            0 => EncodingRecord::get_byte_encoded_glyphs(length, buf),
            4 => EncodingRecord::get_segment_encoded_glyphs(length, buf),

            _ => todo!()
        }
    }

    fn get_byte_encoded_glyphs<T>(_: u16, buf: &mut T) -> std::io::Result<GlyphMap>
        where T: Read + Seek
    {
        let mut glyphs = GlyphMap::new();

        buf.bytes()
            .take(256)
            .enumerate()
            .try_for_each(|(codepoint, glyph)| {
                let Some(character) = char::from_u32(codepoint as u32) else {
                    return Err(new_error!("Failed to convert u64 to codepoint"));
                };

                if glyphs.insert(character, glyph? as u16).is_some() {
                    return Err(new_error!("Failed to insert glyph into table for codepoint {}", codepoint));
                }

                Ok(())
            })?;

        Ok(glyphs)
    }

    fn get_segment_encoded_glyphs<T>(_: u16, buf: &mut T) -> std::io::Result<GlyphMap> 
        where T: Read + Seek
    {
        let segments: usize = (((buf.bytes().next().unwrap_or(Err(new_error!("Unwrapping failed")))? as usize) << 8) |
                            (buf.bytes().next().unwrap_or(Err(new_error!("Unwrapping failed")))? as usize)) >> 1;

        buf.seek(SeekFrom::Current(6))?;

        let mut buffer = vec![0u8; segments as usize * 2];
        
        buf.read_exact(&mut buffer)?;
        let end_codes: Vec<u16> = (0..segments).map(|i| u16::from_be_bytes(buffer[2*(i as usize)..2*(i as usize)+2].try_into().unwrap())).collect();

        buf.seek(SeekFrom::Current(2))?;

        buf.read_exact(&mut buffer)?;
        let start_codes: Vec<u16> = (0..segments).map(|i| u16::from_be_bytes(buffer[2*(i as usize)..2*(i as usize)+2].try_into().unwrap())).collect();

        buf.read_exact(&mut buffer)?;
        let deltas: Vec<i16> = (0..segments).map(|i| i16::from_be_bytes(buffer[2*(i as usize)..2*(i as usize)+2].try_into().unwrap())).collect();

        buf.read_exact(&mut buffer)?;
        let offsets: Vec<u16> = (0..segments).map(|i| u16::from_be_bytes(buffer[2*(i as usize)..2*(i as usize)+2].try_into().unwrap())).collect();

        let glyphs_start = buf.stream_position()?;
        let mut glyphs = GlyphMap::new();

        for range in 0..segments {
            for codepoint in start_codes[range]..=end_codes[range] {
                let glyph: u16;

                if offsets[range] != 0 {
                    let glyph_id = offsets[range] / 2 + (codepoint - start_codes[range]);
                    buf.seek(SeekFrom::Start(glyphs_start + glyph_id as u64))?;

                    let raw_glyph = ((buf.bytes().next().unwrap_or(Err(new_error!("Unwrapping failed")))? as u16) << 8) |
                                         (buf.bytes().next().unwrap_or(Err(new_error!("Unwrapping failed")))? as u16);

                    glyph = raw_glyph.wrapping_add_signed(deltas[range]);

                } else {
                    let glyph_id = (codepoint - start_codes[range]).wrapping_add_signed(deltas[range]);
                    buf.seek(SeekFrom::Start(glyphs_start + glyph_id as u64))?;

                    glyph = ((buf.bytes().next().unwrap_or(Err(new_error!("Unwrapping failed")))? as u16) << 8) |
                            (buf.bytes().next().unwrap_or(Err(new_error!("Unwrapping failed")))? as u16);

                }

                let Some(character) = char::from_u32(codepoint as u32) else {
                    return Err(new_error!("Character conversion failed"));
                };

                glyphs.insert(character, glyph);
            }
        }

        Ok(glyphs)
    }
}
