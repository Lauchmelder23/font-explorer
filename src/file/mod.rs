mod font;
mod error;
mod loader;

use std::io::Read;
use bincode::Options;

pub use font::OpenTypeFont;
use serde::Deserialize;
mod table;

mod outlines;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct BoundingBox {
    pub left: i16,
    pub bottom: i16,
    pub right: i16,
    pub top: i16
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16
}

pub fn deserialize_from<T, S>(stream: &mut S) -> bincode::Result<T>
    where S: Read,
          T: serde::de::DeserializeOwned
{
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
        .deserialize_from(stream.by_ref())
}

pub fn deserialize_vec_from<T, S>(elems: usize, stream: &mut S) -> bincode::Result<Vec<T>>
    where S: Read,
          T: serde::de::DeserializeOwned
{
    if elems == 0 {
        return Ok(Vec::new());
    }

    let mut result: Vec<T> = Vec::with_capacity(elems);

    for _ in 0..elems {
        result.push(deserialize_from(stream)?);
    }

    Ok(result)
}
