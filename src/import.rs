use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::config::Config;
use crate::img::{img_to_meta, Img};
use crate::os::find_files;

//
// Steps to IMPORT in img-DB:
// - find all files
// - filter out unwanted files
// - process each and every image (disk, size, type, color, exif, hashes, etc)
// - write image data on disk
//
pub fn op_import(input: &[PathBuf], cfg: &Config) {
    log::debug!("Using {cfg:?}");

    // validate config by creating a blank image, using the config
    let v = Img::new_blank(&cfg);
    if !v.is_valid() {
        log::error!("Invalid config!");
        return;
    }

    let mut stream = File::options()
        .create(true)
        .append(true)
        .open(&cfg.dbname)
        .unwrap();

    let pths = find_files(&input, &cfg);
    let imgs = pths
        .par_iter()
        .map(|p| -> Vec<_> {
            let i = img_to_meta(p.to_str().unwrap(), &cfg);
            if i.is_null() {
                return Vec::new();
            }
            serde_json::to_vec(&i).unwrap()
        })
        .collect::<Vec<_>>();

    for i in imgs {
        stream
            .write_all(&i)
            .expect("Couldn't write img into DB");
        stream.write_all(b"\n").unwrap();
    }

    stream.flush().unwrap();
}
