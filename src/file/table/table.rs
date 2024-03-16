use std::io::{Read, Seek};

use crate::file::{error::Result, loader::TableDirectoryEntry};

pub trait Table: Sized {
    fn load<S>(entry: TableDirectoryEntry, stream: &mut S) -> Result<Self>
        where S: Read + Seek;
}
