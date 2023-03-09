use glob::glob;
use std::path::PathBuf;

use crate::cli::Cli;

pub fn find_files(pths: &[PathBuf], opts: &Cli) -> Vec<PathBuf> {
    let mut stop = false;
    let mut files: Vec<PathBuf> = Vec::new();

    for fd in pths {
        if stop {
            break;
        }
        // FD is either a file, a dir, or a broken path
        if !fd.exists() {
            println!("Invalid path: {:?}", fd);
            continue;
        }
        if fd.is_file() {
            files.push(fd.to_path_buf());
            if opts.limit > 0 && files.len() == opts.limit {
                stop = true;
            }
        } else if fd.is_dir() {
            let pat = fd.join("*.*");
            for found in glob(&pat.to_string_lossy()).expect("Failed to read glob pattern") {
                match found {
                    Ok(pth) => {
                        if pth.is_file() {
                            files.push(pth);
                        }
                    }
                    Err(e) => println!("Glob err: {:?}", e),
                }
                if opts.limit > 0 && files.len() == opts.limit {
                    stop = true;
                    break;
                }
            }
        } else {
            println!("Invalid file kind: {:?}", fd);
        }
    }
    files
}
