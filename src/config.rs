// use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumString;

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
    pub thumb_sz: u16,
    pub thumb_qual: u8,
    pub thumb_type: ThumbType,
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

/// Allowed thumb types
#[derive(Clone, PartialEq, Eq, Debug, Default, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ThumbType {
    #[default]
    WebP,
    JPEG,
    PNG,
}

impl fmt::Display for ThumbType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ThumbType::WebP => write!(f, "webp"),
            ThumbType::JPEG => write!(f, "jpeg"),
            ThumbType::PNG => write!(f, "png"),
        }
    }
}
