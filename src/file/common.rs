use std::{fs::File, io::BufReader};

macro_rules! new_error {
    ($msg: literal) => (std::io::Error::new(std::io::ErrorKind::Other, $msg));

    ($msg: literal, $($arg: expr),*) => {
        std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!($msg, $($arg),*)
        )
    }
}

pub(crate) use new_error;

pub fn open_file(path: &str) ->  std::io::Result<BufReader<File>> {
    let file = BufReader::new(File::open(path)?);

    Ok(file)
}
