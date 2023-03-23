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
}

/// First subcommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "import")]
pub struct ImportArgs {
    /// input files
    #[argh(option, short = 'i')]
    pub input: Vec<PathBuf>,
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
}
