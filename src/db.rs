use serde_json;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::img::Img;

pub fn db_open(fname: &str) -> Vec<Img> {
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

/// Quickly sort lines and remove duplicates from a file
pub fn db_fast_optimize(fname: &str) {
    let mut lines: Vec<_> = read_lines(&fname)
        .map(|l| l.expect("Couldn't read a line from DB"))
        .collect();

    log::debug!("DB initial: {} lines", lines.len());
    lines.sort();
    lines.dedup();
    log::debug!("DB final: {} lines", lines.len());

    let mut file = File::create(fname).unwrap();

    for line in lines {
        if line.len() < 2 {
            continue;
        }
        file.write_all(line.as_bytes())
            .expect("Couldn't write line into DB");
        file.write_all(b"\n").unwrap();
    }

    file.flush().unwrap();
}

/// Slowly parse each line into Img objects and remove duplicates
pub fn db_slow_compact() {
    //
}

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file
    return BufReader::new(file).lines();
}
