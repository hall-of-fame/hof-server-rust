use regex::Regex;
use rocket::serde::Serialize;
use std::fs;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct MPSticker {
    author: String,
    desc: String,
    url: String,
}

pub fn get_multiple() -> Vec<MPSticker> {
    let images = fs::read_dir("./src/images/Multiple").unwrap().map(|res| res.unwrap());
    let mut stickers = Vec::<MPSticker>::new();
    for image in images {
        let image_name = image.file_name().into_string().unwrap();
        stickers.push(resolve_mpsticker_filename(image_name));
    }
    stickers
}

fn resolve_mpsticker_filename(filename: String) -> MPSticker {
    let reg = Regex::new(r"(.*?)-(.*)\..*$").unwrap();
    let caps = reg.captures(filename.as_str()).unwrap();
    let author = caps.get(1).unwrap().as_str().to_string();
    let desc = caps.get(2).unwrap().as_str().to_string();
    let url = format!("./src/images/multiple/{}", filename.replace(" ", "%20"));
    MPSticker {
        author: author,
        desc: desc,
        url: url,
    }
}
