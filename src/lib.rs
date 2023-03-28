#![deny(unsafe_code)]
#![allow(non_snake_case)]
#![warn(unused_extern_crates)]

pub mod cli;
pub mod config;
pub mod os;

pub mod db;
pub mod gallery;
pub mod img;

mod hashc;
mod hashv;
