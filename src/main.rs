
use NexusCLI::cli::parser::CliArgs;
use NexusCLI::operations::delete::delete;
use NexusCLI::operations::upload::upload;
use clap::Parser;

fn main() {
    let args = CliArgs::parse();
    match args.operation.as_str() {
        "U" => upload(args),
        "D" => delete(args),
        _ => eprintln!("Opération non reconnue"),
    }
}
