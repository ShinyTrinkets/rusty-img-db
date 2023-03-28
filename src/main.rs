#![allow(non_snake_case)]

use argh;
use serde_json;

use imgDB::cli::{Cli, Commands};
use imgDB::config::Config;
use imgDB::db::db_open;
use imgDB::gallery::generate_gallery;
use imgDB::img::{img_to_meta, Img};
use imgDB::os::find_files;

fn main() {
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
            // println!("USING CFG {cfg:?}");
            // validate config by creating a blank image, using the config
            let v = Img::new_blank(&cfg);
            if !v.is_valid() {
                println!("Invalid config!\n");
                return;
            }
            let pths = find_files(&input, &cfg);
            for p in pths {
                let i = img_to_meta(p.to_str().unwrap(), &cfg);
                println!("{}", serde_json::to_string(&i).unwrap());
            }
        }
        Commands::Gallery(cmd) => {
            let fname = &cmd.dbname.clone();
            let cfg = Config::from(cmd);
            let imgs = db_open(fname, &cfg);
            generate_gallery(imgs, &cfg);
        }
        Commands::Links(cmd) => {
            let cfg = Config::from(cmd);
            print!("NOT IMPLEMENTED!\n{:?}", cfg);
        }
    }
}
