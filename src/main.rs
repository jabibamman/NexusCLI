mod cli;
mod operations;
mod utils;

use crate::cli::parser::CliArgs;
use crate::operations::delete::delete;
use crate::operations::upload::upload;
use clap::Parser;

include!(concat!(env!("OUT_DIR"), "/domain.rs"));

fn main() {
    let args = CliArgs::parse();
    match args.operation.as_str() {
        "U" => {
            if args.source.is_none() {
                eprintln!("L'argument '--source' ou '-s' est requis lorsque l'opération est de type 'upload (U)''");
                std::process::exit(1);
            }

            upload(args)
        },
        "D" => delete(args),
        _ => eprintln!("Opération non reconnue"),
    }
}
