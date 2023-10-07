mod operations;
mod cli;
mod utils;

use clap::{Parser};
use crate::cli::parser::CliArgs;
use crate::operations::delete::delete;
use crate::operations::upload::upload;

include!(concat!(env!("OUT_DIR"), "/domain.rs"));

fn main() {
    let args = CliArgs::parse();
    match args.operation.as_str() {
        "U" => upload(args),
        "D" => delete(args),
        _ => eprintln!("Opération non reconnue"),
    }
}
