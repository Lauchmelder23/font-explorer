use std::io::{Read, Seek};

use itertools::izip;
use log::{debug, warn};
use serde::{de::DeserializeOwned, Deserialize};

use crate::file::{self, deserialize_from, error::{FontError, Result}};

use super::CharacterMap;

#[derive(Debug, Clone, Copy, Deserialize)]
struct SubtableHeader {
    length:         u16,
    language:       u16,
    seg_count_x2:   u16,
    _unused:        [u16; 3]
}

fn read_table<S, T>(entries: u16, stream: &mut S) -> Result<Vec<T>> 
    where S: Read + Seek,
          T: Sized + DeserializeOwned
{
    let mut result: Vec<T> = Vec::with_capacity(entries as usize);

    for _ in 0..entries {
        result.push(deserialize_from(stream)?);
    }

    Ok(result)
}

fn process_segment<F>(start: u16, end: u16, map: &mut CharacterMap, mut indexing_func: F) -> Result<()>
    where F: FnMut(u16) -> Result<u16>
{
    for codepoint in start..=end {
        let Some(character) = char::from_u32(codepoint as u32) else {
            warn!("Codepoint 0x{:04x} could not be converted to a character (invalid unicode). Skipping.", codepoint);
            continue;
        };

        let index = indexing_func(codepoint)?;
        if let Some(old_val) = map.insert(character, index) {
            warn!("Codepoint 0x{:04x} was mapped twice. Replaced previous glyph index (0x{:04x}) with new value (0x{:04x}).", codepoint, old_val, index);
        }
    }

    Ok(())
}

/// Implements decoding for CMAP table format 4
pub fn load<S>(stream: &mut S) -> Result<CharacterMap> 
    where S: Read + Seek 
{
    debug!("loading a delta encoded char map at 0x{:08x}", stream.stream_position()?);

    let header: SubtableHeader = deserialize_from(stream)?;
    let num_segments = header.seg_count_x2 / 2;

    debug!("Reading {} entries from tables", num_segments);

    let end_table: Vec<u16>     = read_table(num_segments, stream)?;
    let _: u16                  = deserialize_from(stream)?;
    let start_table: Vec<u16>   = read_table(num_segments, stream)?;
    let delta_table: Vec<i16>   = read_table(num_segments, stream)?;
    let offsets_table: Vec<u16> = read_table(num_segments, stream)?;

    let glyphs_start = stream.stream_position()?;
    let mut char_map = CharacterMap::new();

    debug!("segments found: ");
    for (i, (start, end, delta, offset)) in izip!(start_table, end_table, delta_table, offsets_table).enumerate() {
        debug!("    + {:04X}-{:04X} ({} codepoints)\toffset={},\tdelta={}", start, end, end-start+1, offset, delta);
        if offset == 0 {
            process_segment(start, end, &mut char_map, |codepoint: u16| Ok::<u16, FontError>(codepoint.wrapping_add_signed(delta)))?;
        } else {
            process_segment(start, end, &mut char_map, |codepoint: u16| {
                let glyph_offset = offset + (codepoint - start) - (i as u16);
                stream.seek(std::io::SeekFrom::Start(glyphs_start + glyph_offset as u64))?;
                
                let mut glyph_index: u16 = file::deserialize_from(stream)?;
                if glyph_index != 0 {
                    glyph_index = glyph_index.wrapping_add_signed(delta);
                }

                Ok::<u16, FontError>(glyph_index)
            })?;
        }
    }

    Ok(char_map)
}
