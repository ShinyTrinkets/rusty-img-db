use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

use crate::cli::Cli;
use crate::hashc;

/// High level function to extract all possible data from a image path
pub fn img_to_meta(opts: Cli) -> Img {
    let mut raw: Vec<u8> = Vec::new();
    let pth = String::from(opts.img);
    fs::File::open(pth).unwrap().read_to_end(&mut raw).unwrap();

    let mut hashc = HashMap::new();
    if opts.chash != None {
        for h in opts.chash.unwrap() {
            hashc.insert(h.to_string().to_ascii_lowercase(), hashc::hash_c(h, &raw));
        }
    }

    Img {
        hashc: hashc,
        ..raw_to_meta(raw)
    }
}

/// High level function to extract data from a raw image vector
pub fn raw_to_meta(raw: Vec<u8>) -> Img {
    let meta = rexiv2::Metadata::new_from_buffer(&raw).unwrap();
    println!("TYPE {}", meta.get_media_type().unwrap());

    if meta.has_exif() {
        for t in meta.get_exif_tags().unwrap() {
            if t.contains("Exif.Thumbnai") {
                println!("EXIF {}", t);
            } else {
                println!("EXIF {} = {}", &t, meta.get_tag_string(&t).unwrap());
            }
        }

        println!("exposure {}", meta.get_exposure_time().unwrap());
        println!("fnumber {}", meta.get_fnumber().unwrap());
        println!("focal_length {}", meta.get_focal_length().unwrap());
        println!("ISO {}", meta.get_iso_speed().unwrap());
    }

    if meta.has_iptc() {
        for t in meta.get_iptc_tags().unwrap() {
            println!("IPTC {}  =  {}", &t, meta.get_tag_string(&t).unwrap());
        }
    }

    if meta.has_xmp() {
        for t in meta.get_xmp_tags().unwrap() {
            println!("XMP {}  =  {}", &t, meta.get_tag_string(&t).unwrap());
        }
    }

    let reader = ImageReader::new(Cursor::new(&raw))
        .with_guessed_format()
        .unwrap();

    let img_format = reader.format().unwrap();
    // let img = reader.decode().unwrap();
    // let img_color = img.color();

    Img {
        date: String::from("today"),
        width: 0,
        height: 0,
        color: "X".to_string(),
        // width: img.width() as u16,
        // height: img.height() as u16,
        // color: format!("{img_color:#?}").to_uppercase(),
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
