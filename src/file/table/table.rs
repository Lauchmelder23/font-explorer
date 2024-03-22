use std::io::{Read, Seek};

use log::debug;

use crate::file::{error::Result, loader::TableDirectoryEntry};

pub trait Table: Sized {
    type UserArgsType: Sized;

    fn load<S>(entry: TableDirectoryEntry, stream: &mut S, user_data: Self::UserArgsType) -> Result<Self>
        where S: Read + Seek
    {
        debug!("loading table '{}' at {:#08x}", Self::get_table_name(), entry.offset);
        stream.seek(std::io::SeekFrom::Start(entry.offset as u64))?;

        Self::load_impl(entry, stream, user_data)
    }

    fn get_table_name() -> &'static str;
    fn load_impl<S>(entry: TableDirectoryEntry, stream: &mut S, user_data: Self::UserArgsType) -> Result<Self>
        where S: Read + Seek;
}
