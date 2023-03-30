use argh::FromArgs;
use std::path::PathBuf;

use crate::hashc::HashC;
use crate::hashv::HashV;

/// CLI options
#[derive(FromArgs, PartialEq, Debug)]
pub struct Cli {
    #[argh(subcommand)]
    pub nested: Commands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Commands {
    Import(ImportArgs),
    Gallery(GalleryArgs),
    Links(LinksArgs),
}

/// Import sub-command
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "import")]
pub struct ImportArgs {
    /// input files
    #[argh(option, short = 'i')]
    pub input: Vec<PathBuf>,
    // output
    /// database file to use
    #[argh(option, short = 'x', default = "String::from(\"\")")]
    pub dbname: String,

    /// allowed extensions
    #[argh(option)]
    pub exts: Vec<String>,
    /// limit files
    #[argh(option, default = "0")]
    pub limit: u16,

    /// scan deep
    #[argh(switch)]
    pub deep: bool,
    /// shuffle files
    #[argh(switch)]
    pub shuffle: bool,

    /// crypto hashes
    #[argh(option)]
    pub chash: Vec<HashC>,
    /// visual hashes
    #[argh(option)]
    pub vhash: Vec<HashV>,

    /// thumb size
    #[argh(option, default = "0")]
    pub thumb_sz: u16,
    /// thumb quality
    #[argh(option, default = "0")]
    pub thumb_qual: u8,
    // pub thumb_type: ThumbType

    /// log only warn & error
    #[argh(switch)]
    pub silent: bool,
}

/// Gallery sub-command
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "gallery")]
pub struct GalleryArgs {
    /// database file to use
    #[argh(option, short = 'x')]
    pub dbname: String,
    /// output file
    #[argh(option)]
    pub output: String,

    /// log only warn & error
    #[argh(switch)]
    pub silent: bool,
}

/// Links sub-command
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "links")]
pub struct LinksArgs {
    /// database file to use
    #[argh(option, short = 'x')]
    pub dbname: String,
    /// output file
    #[argh(option)]
    pub output: String,

    /// log only warn & error
    #[argh(switch)]
    pub silent: bool,
}
