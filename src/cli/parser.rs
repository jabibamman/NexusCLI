use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(short, long, required = true)]
    pub operation: String,

    #[clap(short, long)]
    pub repository: Option<String>,

    #[clap(short = 'd', long = "directory")]
    pub directory: Option<String>,

    #[clap(short = 's', long = "source")]
    pub source: Option<String>,
}