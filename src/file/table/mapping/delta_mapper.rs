use std::io::{Read, Seek};

use log::debug;

use crate::file::error::Result;

use super::CharacterMap;

/// Implements decoding for CMAP table format 4
pub fn load<T>(stream: &mut T) -> Result<CharacterMap> 
    where T: Read + Seek 
{
    debug!("loading a delta encoded char map at 0x{:08x}", stream.stream_position()?);
    todo!()
}
