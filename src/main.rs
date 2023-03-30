#![allow(non_snake_case)]

use argh;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use imgDB::cli::{Cli, Commands};
use imgDB::config::Config;
use imgDB::db::{db_fast_optimize, db_open};
use imgDB::gallery::generate_gallery;
use imgDB::import::op_import;

fn main() {
    SimpleLogger::new()
        .without_timestamps()
        .with_colors(true)
        .init()
        .unwrap();

    let cli: Cli = argh::from_env();

    match cli.nested {
        Commands::Import(cmd) => {
            if cmd.silent {
                log::set_max_level(LevelFilter::Warn);
            }
            let input = &cmd.input.clone();
            let cfg = Config::default().merge(Config::from(cmd));
            op_import(input, &cfg);
            // quickly sort lines and remove duplicates
            db_fast_optimize(&cfg.dbname);
        }
        Commands::Gallery(cmd) => {
            if cmd.silent {
                log::set_max_level(LevelFilter::Warn);
            }
            let dbname = &cmd.dbname.clone();
            let cfg = Config::from(cmd);
            let imgs = db_open(dbname);
            generate_gallery(imgs, &cfg);
        }
        Commands::Links(cmd) => {
            let cfg = Config::from(cmd);
            log::error!("NOT IMPLEMENTED!\n{cfg:?}");
        }
    }
}
