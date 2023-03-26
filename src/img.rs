use base64::{engine::general_purpose, Engine as _};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GrayImage, ImageOutputFormat};
use rexiv2::Metadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

use crate::config::{Config, ThumbType};
use crate::hashc;
use crate::hashv;

/// High level function to extract all possible data from a image path
pub fn img_to_meta(pth: &str, cfg: &Config) -> Img {
    let mut raw: Vec<u8> = Vec::new();
    fs::File::open(pth).unwrap().read_to_end(&mut raw).unwrap();

    let (img, i1) = raw_to_image(&raw, cfg);
    if i1.is_empty() {
        println!("Cannot decode image format!");
        return i1;
    }

    let i2 = raw_to_meta(&raw);
    if i2.is_empty() {
        println!("Cannot read image Metadata!");
        return i2.merge(i1);
    }
    // TODO: assert widths & heights (and others) are the same

    let mut result = i1.merge(i2);
    result.pth = pth.to_string();

    //
    // calculate visual hashes
    if cfg.vhash.len() > 0 {
        // downscale full image using triangle linear filter because it's fast and a bit blurry
        let small_img = img.resize_exact(16, 16, FilterType::Triangle);
        let mut hv: Vec<(String, String)> = Vec::new();
        for h in &cfg.vhash {
            let key = h.to_string().to_ascii_lowercase();
            let val = hashv::hash_v(&h, small_img.grayscale().into_luma8());
            hv.push((key, val));
        }
        result.hashv = hv.into_iter().collect();
    }

    //
    // calculate cryptographic hashes
    if cfg.chash.len() > 0 {
        let mut hc: Vec<(String, String)> = Vec::new();
        for h in &cfg.chash {
            let key = h.to_string().to_ascii_uppercase();
            let val = hashc::hash_c(&h, &raw);
            hc.push((key, val));
        }
        result.hashc = hc.into_iter().collect();
    }

    // TODO: if no date, get from os.stat

    result
}

/// Read and decode a raw image vector
fn raw_to_image(raw: &Vec<u8>, cfg: &Config) -> (DynamicImage, Img) {
    let reader = match ImageReader::new(Cursor::new(&raw)).with_guessed_format() {
        Ok(m) => m,
        _ => {
            let gray: GrayImage = GrayImage::new(1, 1);
            return (DynamicImage::ImageLuma8(gray), Img::default());
        }
    };
    if reader.format() == None {
        let gray: GrayImage = GrayImage::new(1, 1);
        return (DynamicImage::ImageLuma8(gray), Img::default());
    }

    let img_format = reader.format().unwrap();
    let img = reader.decode().unwrap();
    let img_color = img.color();

    let mut result = Img::default();
    result.bytes = raw.len() as u32;
    result.width = img.width() as u16;
    result.height = img.height() as u16;
    result.color = format!("{img_color:?}").to_uppercase();
    result.format = format!("{img_format:?}").to_uppercase();
    result.b64 = image_thumb(&img, &cfg);

    (img, result)
}

fn image_thumb(img: &DynamicImage, cfg: &Config) -> String {
    let thumb_type = match cfg.thumb_type {
        ThumbType::WebP => ImageOutputFormat::WebP,
        ThumbType::JPEG => ImageOutputFormat::Jpeg(cfg.thumb_qual),
        ThumbType::PNG => ImageOutputFormat::Png,
    };
    let mut thumb_data: Vec<u8> = Vec::new();
    let thumb = img.thumbnail(cfg.thumb_sz as u32, cfg.thumb_sz as u32);
    thumb
        .write_to(&mut Cursor::new(&mut thumb_data), thumb_type)
        .unwrap();
    let thumb_base64 = general_purpose::STANDARD_NO_PAD.encode(thumb_data);
    format!("data:image/{};base64,{}", cfg.thumb_type, thumb_base64)
}

/// Extract meta-data from a raw image vector
fn raw_to_meta(raw: &Vec<u8>) -> Img {
    let meta = match rexiv2::Metadata::new_from_buffer(&raw) {
        Ok(m) => m,
        _ => return Img::default(),
    };

    let mime_type = meta.get_media_type().unwrap();
    let mut img_meta = ImgMeta::default();

    let mut result = Img::default();
    result.bytes = raw.len() as u32;
    result.width = meta.get_pixel_width() as u16;
    result.height = meta.get_pixel_height() as u16;
    result.mime = format!("{mime_type}");

    if meta.has_exif() {
        // for t in meta.get_exif_tags().unwrap() {
        //     if t.contains(".Thumbnail.") {
        //         println!("EXIF {}", t);
        //     } else {
        //         println!("EXIF {} = {}", &t, meta.get_tag_string(&t).unwrap());
        //     }
        // }

        result.date = get_img_date(&meta);

        img_meta.maker = match meta.get_tag_string("Exif.Image.Make") {
            Ok(v) => v,
            _ => String::from(""),
        };
        img_meta.model = match meta.get_tag_string("Exif.Image.Model") {
            Ok(v) => v,
            _ => String::from(""),
        };
        img_meta.lens_maker = match meta.get_tag_string("Exif.Photo.LensMake") {
            Ok(v) => v,
            _ => String::from(""),
        };
        img_meta.lens_model = match meta.get_tag_string("Exif.Photo.LensModel") {
            Ok(v) => v,
            _ => String::from(""),
        };

        img_meta.aperture = match meta.get_fnumber() {
            Some(v) => v.to_string(),
            _ => String::from(""),
        };
        img_meta.shutter_speed = match meta.get_exposure_time() {
            // this is a Ratio, maybe I can improve
            Some(v) => v.to_string(),
            _ => String::from(""),
        };
        img_meta.focal_length = match meta.get_focal_length() {
            Some(v) => v.to_string(),
            _ => String::from(""),
        };
        img_meta.iso = match meta.get_iso_speed() {
            Some(v) => v as u32,
            _ => 0,
        };
    } else {
        println!("No EXIF");
    }

    // if meta.has_iptc() {
    //     for t in meta.get_iptc_tags().unwrap() {
    //         println!("IPTC {}  =  {}", &t, meta.get_tag_string(&t).unwrap());
    //     }
    // }

    if meta.has_xmp() {
        // for t in meta.get_xmp_tags().unwrap() {
        //     println!("XMP {}  =  {}", &t, meta.get_tag_string(&t).unwrap());
        // }

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

    result.meta = img_meta;
    result
}

fn get_img_date(meta: &Metadata) -> String {
    let tag_to_date = |s: &str| -> String {
        match meta.get_tag_string(s) {
            Ok(v) => v,
            _ => String::from(""),
        }
    };

    let date = tag_to_date("Exif.Photo.DateTimeOriginal");
    if date.len() > 6 {
        return date;
    }
    let date = tag_to_date("Exif.Photo.DateTimeDigitized");
    if date.len() > 6 {
        return date;
    }
    let date = tag_to_date("Exif.Photo.DateTime");
    if date.len() > 6 {
        return date;
    }
    let date = tag_to_date("Exif.Photo.DateTime");
    if date.len() > 6 {
        return date;
    }

    // XMP:DateCreated
    // IPTC:DateCreated

    String::from("")
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct Img {
    pth: String,
    b64: String,
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
    hashv: HashMap<String, String>,
    meta: ImgMeta,
}

impl Img {
    fn merge(self, other: Img) -> Self {
        Self {
            pth: if self.pth != "" { self.pth } else { other.pth },
            b64: if self.b64 != "" { self.b64 } else { other.b64 },
            date: if self.date != "" {
                self.date
            } else {
                other.date
            },
            color: if self.color != "" {
                self.color
            } else {
                other.color
            },
            format: if self.format != "" {
                self.format
            } else {
                other.format
            },
            mime: if self.mime != "" {
                self.mime
            } else {
                other.mime
            },
            width: if self.width > 0 {
                self.width
            } else {
                other.width
            },
            height: if self.height > 0 {
                self.height
            } else {
                other.height
            },
            bytes: if self.bytes > 0 {
                self.bytes
            } else {
                other.bytes
            },
            hashc: self.hashc.into_iter().chain(other.hashc).collect(),
            hashv: self.hashv.into_iter().chain(other.hashv).collect(),
            meta: self.meta.merge(other.meta),
        }
    }

    fn is_empty(&self) -> bool {
        self.bytes == 0 && self.width == 0 && self.height == 0
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
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
    fn merge(self, other: ImgMeta) -> Self {
        Self {
            maker: if self.maker != "" {
                self.maker
            } else {
                other.maker
            },
            model: if self.model != "" {
                self.model
            } else {
                other.model
            },
            lens_maker: if self.lens_maker != "" {
                self.lens_maker
            } else {
                other.lens_maker
            },
            lens_model: if self.lens_model != "" {
                self.lens_model
            } else {
                other.lens_model
            },
            aperture: if self.aperture != "" {
                self.aperture
            } else {
                other.aperture
            },
            shutter_speed: if self.shutter_speed != "" {
                self.shutter_speed
            } else {
                other.shutter_speed
            },
            focal_length: if self.focal_length != "" {
                self.focal_length
            } else {
                other.focal_length
            },
            iso: if self.iso > 0 { self.iso } else { other.iso },
            rating: if self.rating > 0 {
                self.rating
            } else {
                other.rating
            },
            label: if self.label != "" {
                self.label
            } else {
                other.label
            },
            subject: if self.subject != "" {
                self.subject
            } else {
                other.subject
            },
            keywords: if self.keywords != "" {
                self.keywords
            } else {
                other.keywords
            },
            headline: if self.headline != "" {
                self.headline
            } else {
                other.headline
            },
            caption: if self.caption != "" {
                self.caption
            } else {
                other.caption
            },
        }
    }
}
