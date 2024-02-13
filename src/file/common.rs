use std::{fs::File, io::BufReader};

pub fn open_file(path: &str) ->  std::io::Result<BufReader<File>> {
    let file = BufReader::new(File::open(path)?);

    Ok(file)
}
