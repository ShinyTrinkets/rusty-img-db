// use serde::{Deserialize, Serialize};

use crate::cli::*;
use crate::hashc::HashC;
use crate::hashv::HashV;

pub fn default_config() -> Config {
    Config {
        dbname: String::from("imgdb.htm"),
        thumb_sz: 128,
        thumb_qual: 70,
        ..Default::default()
    }
}

/// App common config
#[derive(PartialEq, Eq, Debug, Default)]
pub struct Config {
    pub dbname: String,

    // /// input: a list or files, or folders
    // pub inputs: Vec<String>,
    /// output: the output folder
    pub output: String,

    /// limit files
    pub limit: u16,
    /// scan folders deep?
    pub deep: bool,
    /// shuffle files?
    pub shuffle: bool,

    /// crypto hashes
    pub chash: Vec<HashC>,
    /// visual hashes
    pub vhash: Vec<HashV>,

    /// thumb size, quality and type
    thumb_sz: u16,
    thumb_qual: u8,
    thumb_type: String,
}

impl From<ImportArgs> for Config {
    fn from(args: ImportArgs) -> Config {
        let cfg = default_config();
        Config {
            // inputs: args.input,
            limit: args.limit,
            deep: args.deep,
            shuffle: args.shuffle,
            chash: args.chash,
            vhash: args.vhash,
            ..cfg
        }
    }
}

impl From<GalleryArgs> for Config {
    fn from(args: GalleryArgs) -> Config {
        Config {
            dbname: args.dbname,
            output: args.output,
            ..default_config()
        }
    }
}

impl From<LinksArgs> for Config {
    fn from(args: LinksArgs) -> Config {
        Config {
            dbname: args.dbname,
            output: args.output,
            ..default_config()
        }
    }
}
