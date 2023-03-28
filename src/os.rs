use glob::glob;
use glob::GlobError;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;

use crate::config::Config;

pub fn find_files(pths: &[PathBuf], cfg: &Config) -> Vec<PathBuf> {
    let mut stop = false;
    let mut rng = thread_rng();
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
            if cfg.limit > 0 && files.len() as u16 == cfg.limit as u16 {
                stop = true;
            }
        } else if fd.is_dir() {
            let pat = if cfg.deep {
                fd.join("**/*.*")
            } else {
                fd.join("*.*")
            };

            let mut found: Vec<_> = glob(&pat.to_string_lossy())
                .unwrap()
                .map(|x: Result<PathBuf, GlobError>| x.unwrap())
                .collect();
            if cfg.shuffle {
                found.shuffle(&mut rng);
            } else {
                found.sort();
            }

            for p in found {
                if cfg.exts.len() > 0 {
                    let ext = p
                        .extension()
                        .unwrap()
                        .to_ascii_lowercase()
                        .into_string()
                        .unwrap();
                    if !cfg.exts.contains(&ext) {
                        continue;
                    }
                }
                if p.is_file() {
                    files.push(p);
                }
                if cfg.limit > 0 && files.len() as u16 == cfg.limit {
                    stop = true;
                    break;
                }
            }
        } else {
            println!("Invalid file kind: {:?}", fd);
        }
    }

    if cfg.shuffle {
        files.shuffle(&mut rng);
    }

    files
}
