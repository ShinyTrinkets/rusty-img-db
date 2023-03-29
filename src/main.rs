#![allow(non_snake_case)]

use argh;
use rayon::prelude::*;
use serde_json;
use simple_logger::SimpleLogger;

use imgDB::cli::{Cli, Commands};
use imgDB::config::Config;
use imgDB::db::db_open;
use imgDB::gallery::generate_gallery;
use imgDB::img::{img_to_meta, Img};
use imgDB::os::find_files;

fn main() {
    SimpleLogger::new()
        .without_timestamps()
        .with_colors(true)
        .init()
        .unwrap();
    let cli: Cli = argh::from_env();
    match cli.nested {
        // Steps to IMPORT to img-DB:
        // - find all files
        // - filter out files that are not wanted
        // - process each and every image (disk, size, type, color, exif, hashes, etc)
        // - write image data to disk
        Commands::Import(cmd) => {
            // IDEA: the import logic could be a separate file
            let input = &cmd.input.clone();
            let cfg = Config::default().merge(Config::from(cmd));
            log::debug!("Using {cfg:?}");
            // validate config by creating a blank image, using the config
            let v = Img::new_blank(&cfg);
            if !v.is_valid() {
                log::error!("Invalid config!");
                return;
            }
            let pths = find_files(&input, &cfg);
            pths.par_iter().for_each(|p| {
                let i = img_to_meta(p.to_str().unwrap(), &cfg);
                if i.is_null() {
                    return;
                }
                println!("{}", serde_json::to_string(&i).unwrap());
            })
        }
        Commands::Gallery(cmd) => {
            let fname = &cmd.dbname.clone();
            let cfg = Config::from(cmd);
            let imgs = db_open(fname, &cfg);
            generate_gallery(imgs, &cfg);
        }
        Commands::Links(cmd) => {
            let cfg = Config::from(cmd);
            log::error!("NOT IMPLEMENTED!\n{:?}", cfg);
        }
    }
}
