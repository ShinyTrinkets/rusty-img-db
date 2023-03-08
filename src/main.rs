#![allow(non_snake_case)]

use clap::Parser;
use imgDB::cli::Cli;
use imgDB::img::img_to_meta;

fn main() {
    // Steps to IMPORT to img-DB:
    // - find all files
    // - filter out files that are not wanted
    // - process each and every image (disk, size, type, color, exif, hashes, etc)
    // - write image data to disk as HTML or whatever
    //
    let cli = Cli::parse();
    println!("IMG: {:?}", cli.img);
    println!("Meta: {:?}", img_to_meta(cli));
}
