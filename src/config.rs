// use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumString;

use crate::cli::*;
use crate::hashc::HashC;
use crate::hashv::HashV;

/// App common config
#[derive(PartialEq, Eq, Debug)]
pub struct Config {
    pub dbname: String,

    // /// input: a list or files, or folders
    // pub inputs: Vec<String>,
    /// output: the output folder
    pub output: String,

    // the UID is used to calculate the uniqueness of the img
    // TODO: validate && sanitize !!
    pub uid: String,

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

impl Config {
    pub fn merge(self, other: Config) -> Self {
        Self {
            uid: if other.uid != "" { other.uid } else { self.uid },
            dbname: if other.dbname != "" {
                other.dbname
            } else {
                self.dbname
            },
            output: if other.output != "" {
                other.output
            } else {
                self.output
            },
            limit: if other.limit > 0 {
                other.limit
            } else {
                self.limit
            },
            deep: if self.deep != other.deep {
                other.deep
            } else {
                self.deep
            },
            shuffle: if self.shuffle != other.shuffle {
                other.shuffle
            } else {
                self.shuffle
            },
            thumb_sz: if other.thumb_sz > 0 {
                other.thumb_sz
            } else {
                self.thumb_sz
            },
            thumb_qual: if other.thumb_qual > 0 {
                other.thumb_qual
            } else {
                self.thumb_qual
            },
            thumb_type: if self.thumb_type != other.thumb_type {
                other.thumb_type
            } else {
                self.thumb_type
            },
            chash: if other.chash.len() > 0 {
                other.chash
            } else {
                self.chash
            },
            vhash: if other.vhash.len() > 0 {
                other.vhash
            } else {
                self.vhash
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            uid: String::from("{{ img.hashc.SHA224 }}"),
            dbname: String::from("imgdb.htm"),
            output: String::from(""),
            limit: 0,
            deep: false,
            shuffle: false,
            thumb_sz: 96,
            thumb_qual: 70,
            thumb_type: ThumbType::WebP,
            chash: vec![HashC::Sha224, HashC::Ripemd128],
            vhash: vec![HashV::Ahash, HashV::Dhash],
        }
    }
}

impl From<ImportArgs> for Config {
    fn from(args: ImportArgs) -> Config {
        let cfg = Config::default();
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
            ..Config::default()
        }
    }
}

impl From<LinksArgs> for Config {
    fn from(args: LinksArgs) -> Config {
        Config {
            dbname: args.dbname,
            output: args.output,
            ..Config::default()
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
