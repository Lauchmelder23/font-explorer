use crate::file::OpenTypeFont;

mod file;

fn main() {
    let my_ttf = match OpenTypeFont::load("/usr/share/fonts/TTF/Arial.TTF") {
        Ok(val) => val,
        Err(err) => { 
            println!("{}", err);
            return;
        }
    };

    dbg!(my_ttf);
}
