#![allow(non_snake_case)]

use argh;
use serde_json;

use imgDB::cli::{Cli, Commands};
use imgDB::config::Config;
use imgDB::img::img_to_meta;
use imgDB::os::find_files;

fn main() {
    // Steps to IMPORT to img-DB:
    // - find all files
    // - filter out files that are not wanted
    // - process each and every image (disk, size, type, color, exif, hashes, etc)
    // - write image data to disk as HTML or whatever
    //
    let cli: Cli = argh::from_env();
    match cli.nested {
        Commands::Import(cmd) => {
            let input = &cmd.input.clone();
            let cfg = Config::from(cmd);
            let pths = find_files(input, &cfg);
            for p in pths {
                let i = img_to_meta(p.to_str().unwrap(), &cfg);
                println!("{}", serde_json::to_string(&i).unwrap());
            }
        }
    }
}
