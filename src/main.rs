use crate::file::OpenFontFile;

mod file;

fn main() {
    let my_ttf = OpenFontFile::load("/usr/share/fonts/TTF/Arial.TTF").unwrap();

    dbg!(my_ttf);
}
