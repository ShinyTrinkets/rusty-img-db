use crate::hashc;
use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

/// High level function to extract all possible data from a image path
pub fn img_to_meta(pth: String) -> Img {
    let mut raw: Vec<u8> = Vec::new();
    fs::File::open(pth).unwrap().read_to_end(&mut raw).unwrap();

    let hashc: HashMap<String, String> = HashMap::from([
        (
            "sha256".to_string(),
            hashc::hash_c(hashc::HashC::Sha256, &raw),
        ),
        (
            "blake256".to_string(),
            hashc::hash_c(hashc::HashC::Blake256, &raw),
        ),
    ]);
    Img {
        hashc: hashc,
        ..raw_to_meta(raw)
    }
}

/// High level function to extract data from a raw image vector
pub fn raw_to_meta(raw: Vec<u8>) -> Img {
    let reader = ImageReader::new(Cursor::new(&raw))
        .with_guessed_format()
        .unwrap();

    let img_format = reader.format().unwrap();
    let img = reader.decode().unwrap();
    let img_color = img.color();

    Img {
        date: String::from("today"),
        width: img.width() as u16,
        height: img.height() as u16,
        color: format!("{img_color:#?}").to_uppercase(),
        format: format!("{img_format:#?}").to_uppercase(),
        bytes: raw.len() as u32,
        hashc: HashMap::new(),
        // hashv: hashv,
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Img {
    // creation date
    date: String,
    color: String,
    format: String,
    // image size width & height
    width: u16,
    height: u16,
    // disk size bytes
    bytes: u32,
    hashc: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImgMeta {
    // Extra stuff from EXIF, IPTC, XMP & ICC Profile
    // This could potentially be a HashMap, to allow flexible fields
    //
    // camera maker & model
    maker_model: String,
    lens_make: String,
    lens_model: String,

    aperture: String,
    shutter_speed: String,
    focal_length: String,
    iso: u32,

    rating: u8,
    label: String,
    keywords: String,
    headline: String,
    caption: String,
}
