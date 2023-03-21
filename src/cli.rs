use clap::Parser;
use std::path::PathBuf;

use crate::hashc::HashC;
use crate::hashv::HashV;

/// CLI options
#[derive(Parser)]
#[command(name = "imgDB")]
#[command(author = "Cristi Constantin")]
#[command(version = "0.1")]
pub struct Cli {
    #[arg(short, long)]
    pub input: Vec<PathBuf>,
    #[arg(long, default_value_t = 0)]
    pub limit: usize,
    #[arg(long, default_value_t = false)]
    pub deep: bool,
    #[arg(long, default_value_t = false)]
    pub shuffle: bool,

    // crypto hashes
    #[arg(long, value_enum)]
    pub chash: Option<Vec<HashC>>,
    // visual hashes
    #[arg(long, value_enum)]
    pub vhash: Option<Vec<HashV>>,
}
