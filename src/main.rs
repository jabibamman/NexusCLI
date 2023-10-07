mod operations;
mod cli;
mod utils;

use clap::{Parser};
use crate::cli::parser::CliArgs;
use crate::operations::delete::delete;

include!(concat!(env!("OUT_DIR"), "/domain.rs"));



fn main() {
    let args = CliArgs::parse();
    match args.operation.as_str() {
        "U" => println!("Opération upload"),
        "D" => delete(args),
        _ => eprintln!("Opération non reconnue"),
    }
}
