use clap::Parser;
use crate::hashc::HashC;

/// CLI options
#[derive(Parser)]
#[command(name = "imgDB")]
#[command(author = "Cristi Constantin")]
#[command(version = "0.1")]
pub struct Cli {
    #[arg(long)]
    pub img: String,
    #[arg(long, value_enum)]
    pub chash: Option<Vec<HashC>>, // crypto hash
}
// #[arg(long, value_enum)]
// vhash: Option<Vec<HashC>>, // visual hash
