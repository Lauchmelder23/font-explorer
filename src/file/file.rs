use std::{fs::File, io::{BufReader, Bytes, Read}};

use super::common::open_file;

#[derive(Debug)]
pub struct OpenFontFile {
    file: String,
    directory: TableDirectory
}

impl OpenFontFile {
    pub fn load(filepath: &str) -> std::io::Result<OpenFontFile> {
        let mut buf = open_file(filepath)?;
        
        let directory = TableDirectory::read_from_buf(&mut buf)?;

        Ok(OpenFontFile {
            file: String::from(filepath),
            directory
        })
    }

    fn read_table<T>(data: &mut Bytes<T>) -> std::io::Result<()>
        where T: std::io::Read
    {
        let mut tag: u32 = 0;

        for byte in data.take(4) {
            tag = (tag << 8) | (byte? as u32);
        }

        println!("{:#x}", tag);

        Ok(())
    }
}

#[derive(Debug)]
struct TableRecord {
    tag: u32,
    checksum: u32,
    offset: u32,
    length: u32
}

#[derive(Debug)]
struct TableDirectory {
    sfnt_version: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    records: Vec<TableRecord>
}

impl TableDirectory {
    fn read_from_buf(data: &mut BufReader<File>) -> std::io::Result<TableDirectory> {
        let mut buf: [u8; 12] = [0; 12];
        data.read_exact(&mut buf)?;

        let version = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        if version != 0x10000 && version != 0x4F54544F {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "unsupported snft version encountered"));
        }

        let num_tables = u16::from_be_bytes(buf[4..6].try_into().unwrap());
        let search_range = u16::from_be_bytes(buf[6..8].try_into().unwrap());
        let entry_selector = u16::from_be_bytes(buf[8..10].try_into().unwrap());
        let range_shift = u16::from_be_bytes(buf[10..12].try_into().unwrap());

        let mut table_dir = TableDirectory {
            sfnt_version: version,
            num_tables, search_range, entry_selector, range_shift,
            records: vec![]
        };

        table_dir.read_table_records(data)?;

        Ok(table_dir)
    }

    fn read_table_records(&mut self, data: &mut BufReader<File>) -> std::io::Result<()>{
        for _ in 0..self.num_tables {
            let record = read_single_table_record(data)?;
            self.records.push(record);
        }

        Ok(())
    }
}

fn read_single_table_record(data: &mut BufReader<File>) -> std::io::Result<TableRecord> {
    let mut buf: [u8; 4 * 4] = [0; 4 * 4];
    data.read_exact(&mut buf)?;

    Ok(TableRecord {
        tag: u32::from_be_bytes(buf[0..4].try_into().unwrap()),
        checksum: u32::from_be_bytes(buf[4..8].try_into().unwrap()),
        offset: u32::from_be_bytes(buf[8..12].try_into().unwrap()),
        length: u32::from_be_bytes(buf[12..16].try_into().unwrap())
    })
}
