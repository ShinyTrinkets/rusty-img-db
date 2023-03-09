use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

use crate::cli::Cli;
use crate::hashc;

/// High level function to extract all possible data from a image path
pub fn img_to_meta(pth: String, opts: &Cli) -> Img {
    let mut raw: Vec<u8> = Vec::new();
    fs::File::open(pth).unwrap().read_to_end(&mut raw).unwrap();

    let mut hashc = HashMap::new();
    if opts.chash != None {
        for h in opts.chash.as_ref().unwrap() {
            hashc.insert(h.to_string(), hashc::hash_c(&h, &raw));
        }
    }

    Img {
        hashc: hashc,
        ..raw_to_meta(raw)
    }
}

/// High level function to extract data from a raw image vector
fn raw_to_meta(raw: Vec<u8>) -> Img {
    let meta = match rexiv2::Metadata::new_from_buffer(&raw) {
        Ok(m) => m,
        _ => return Img::empty(),
    };

    let mime_type = meta.get_media_type().unwrap();
    let mut img_meta = ImgMeta::empty();

    if meta.has_exif() {
        for t in meta.get_exif_tags().unwrap() {
            if t.contains(".Thumbnail.") {
                println!("EXIF {}", t);
            } else {
                println!("EXIF {} = {}", &t, meta.get_tag_string(&t).unwrap());
            }
        }

        img_meta.maker = meta.get_tag_string("Exif.Image.Make").unwrap();
        img_meta.model = meta.get_tag_string("Exif.Image.Model").unwrap();
        img_meta.lens_maker = meta.get_tag_string("Exif.Photo.LensMake").unwrap();
        img_meta.lens_model = meta.get_tag_string("Exif.Photo.LensModel").unwrap();

        img_meta.aperture = meta.get_fnumber().unwrap().to_string();
        img_meta.shutter_speed = meta.get_exposure_time().unwrap().to_string();
        img_meta.focal_length = meta.get_focal_length().unwrap().to_string();
        img_meta.iso = meta.get_iso_speed().unwrap() as u32;
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
        if meta.has_tag("Xmp.xmp.Rating") {
            img_meta.rating = meta.get_tag_numeric("Xmp.xmp.Rating") as u8;
        }
        if meta.has_tag("Xmp.xmp.Label") {
            img_meta.label = meta.get_tag_string("Xmp.xmp.Label").unwrap();
        }
        if meta.has_tag("Xmp.dc.subject") {
            img_meta.subject = meta.get_tag_string("Xmp.dc.subject").unwrap();
        }
    }

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
        color: format!("{img_color:?}").to_uppercase(),
        format: format!("{img_format:?}").to_uppercase(),
        mime: format!("{mime_type}"),
        bytes: raw.len() as u32,
        hashc: HashMap::new(),
        // hashv: hashv,
        meta: img_meta,
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Img {
    // creation date
    date: String,
    color: String,
    format: String,
    // mime type
    mime: String,
    // image size width & height
    width: u16,
    height: u16,
    // disk size bytes
    bytes: u32,
    hashc: HashMap<String, String>,
    // hashv
    meta: ImgMeta,
}

impl Img {
    fn empty() -> Img {
        Img {
            date: "".to_string(),
            color: "".to_string(),
            format: "".to_string(),
            mime: "".to_string(),
            width: 0,
            height: 0,
            bytes: 0,
            hashc: HashMap::new(),
            // hashv
            meta: ImgMeta::empty(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImgMeta {
    // Extra stuff from EXIF, IPTC, XMP & ICC Profile
    // This could potentially be a HashMap, to allow flexible fields
    //
    // camera & lens, maker & model
    maker: String,
    model: String,
    lens_maker: String,
    lens_model: String,

    // F stops
    aperture: String,
    // exposure time in sec
    shutter_speed: String,
    // focal length in mm
    focal_length: String,
    iso: u32,

    // manual XMP tags
    rating: u8,
    label: String,
    subject: String,
    keywords: String,
    headline: String,
    caption: String,
}

impl ImgMeta {
    fn empty() -> ImgMeta {
        ImgMeta {
            maker: "".to_string(),
            model: "".to_string(),
            lens_maker: "".to_string(),
            lens_model: "".to_string(),
            aperture: "".to_string(),
            shutter_speed: "".to_string(),
            focal_length: "".to_string(),
            iso: 0,
            rating: 0,
            label: "".to_string(),
            subject: "".to_string(),
            keywords: "".to_string(),
            headline: "".to_string(),
            caption: "".to_string(),
        }
    }
}
