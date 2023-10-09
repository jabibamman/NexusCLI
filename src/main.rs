use clap::Parser;
use NexusCLI::cli::parser::CliArgs;
use NexusCLI::operations::delete::delete;
use NexusCLI::operations::upload::upload;

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
