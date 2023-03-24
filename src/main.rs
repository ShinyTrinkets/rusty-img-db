#![allow(non_snake_case)]

use argh;
use serde_json;

use imgDB::cli::{Cli, Commands};
use imgDB::config::Config;
use imgDB::db::db_open;
use imgDB::img::img_to_meta;
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
            let cfg = Config::from(cmd);
            let pths = find_files(input, &cfg);
            for p in pths {
                let i = img_to_meta(p.to_str().unwrap(), &cfg);
                println!("{}", serde_json::to_string(&i).unwrap());
            }
        }
        Commands::Gallery(cmd) => {
            let fname = &cmd.dbname.clone();
            let cfg = Config::from(cmd);
            for img in db_open(fname, &cfg) {
                println!("{:?}", img);
            }
        }
        Commands::Links(cmd) => {
            let cfg = Config::from(cmd);
            print!("NOT IMPLEMENTED!\n{:?}", cfg);
        }
    }
}
