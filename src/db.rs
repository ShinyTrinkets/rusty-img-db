use serde_json;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::config::Config;
use crate::img::Img;

pub fn db_open(fname: &str, _cfg: &Config) -> Vec<Img> {
    let mut imgs: Vec<Img> = Vec::new();
    let lines = read_lines(fname);
    for line in lines {
        let line = line.unwrap();
        match serde_json::from_str(&line) {
            Ok(img) => {
                imgs.push(img);
            }
            Err(err) => {
                eprintln!("ERR parsing DB entry: {err}")
            }
        }
    }
    imgs
}

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file
    return BufReader::new(file).lines();
}
