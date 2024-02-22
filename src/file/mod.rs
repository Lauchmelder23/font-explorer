pub mod file;
mod error;

use std::io::Read;
use bincode::Options;

pub use file::OpenTypeFont;
mod table;

pub fn deserialize_from<T, R>(reader: &mut R) -> bincode::Result<T>
    where R: Read,
          T: serde::de::DeserializeOwned
{
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
        .deserialize_from(reader.by_ref())
}
