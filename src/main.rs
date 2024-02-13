use crate::file::OpenFontFile;

mod file;

fn main() {
    let my_ttf = OpenFontFile::load("/usr/share/fonts/TTF/FiraCode-Regular.ttf").unwrap();
    dbg!(my_ttf);
}
