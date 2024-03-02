use std::error::Error;

use log::{error, info};

use crate::file::OpenTypeFont;

mod file;

fn main() {
    env_logger::init();

    let my_ttf = match OpenTypeFont::load("/usr/share/fonts/TTF/Arial.TTF") {
        Ok(val) => val,
        Err(err) => { 
            error!("{}", err);
            if let Some(inner) = err.source() {
                error!("{}", inner);
            }
            return;
        }
    };

    info!("{:?}", my_ttf);
}
