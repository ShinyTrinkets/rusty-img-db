#![allow(non_snake_case)]

use clap::Parser;
use imgDB::img::img_to_meta;

#[derive(Parser)]
#[command(name = "imgDB")]
#[command(author = "Cristi Constantin")]
#[command(version = "0.1")]
struct Cli {
    #[arg(long)]
    img: String,
}

fn main() {
    // Steps to IMPORT to img-DB:
    // - find all files
    // - filter out files that are not wanted
    // - process each and every image (disk, size, type, color, exif, hashes, etc)
    // - write image data to disk as HTML or whatever
    //
    let cli = Cli::parse();
    println!("IMG: {:?}", cli.img);

    let pth = String::from(cli.img);
    println!("{:?}", img_to_meta(pth));
}
