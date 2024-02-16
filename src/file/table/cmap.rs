use std::io::{BufReader, Read, Seek};
use super::{TableReader, TableRecord};

#[derive(Debug)]
pub struct CharacterGlyphMapping {
    version: u16,
    encodings: Vec<EncodingRecord>
}

impl CharacterGlyphMapping {
    fn read_subtable<T>(data: &[u8], buf: &mut T, record: &TableRecord) -> std::io::Result<EncodingRecord>
        where T: Seek {
        let platform = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let encoding = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let offset = u32::from_be_bytes(data[4..8].try_into().unwrap());

        let pos = record.offset + offset;
        let next_entry_pos = buf.stream_position()?;

        let encoding_record = EncodingRecord::new(platform, encoding, pos);

        buf.seek(std::io::SeekFrom::Start(next_entry_pos))?;

        Ok(encoding_record)
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

#[derive(Debug)]
pub struct EncodingRecord {
    platform: u16,
    encoding: u16,
}

impl EncodingRecord {
    fn new(platform: u16, encoding: u16, pos: u32) -> EncodingRecord {
        EncodingRecord { 
            platform, encoding
        }
    }
}
