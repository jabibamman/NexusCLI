use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(
        short = 'o',
        long = "operation",
        required = true
    )]
    pub operation: String,

    #[clap(
        short = 'r',
        long = "repository",
        required = true,
        default_value = "depot-local-snapshot"
    )]
    pub repository: Option<String>,

    #[clap(
        short = 'd',
        long = "directory",
        required = true,
        default_value = "rp/omer/ihm/homere-DV05.zip"
    )]
    pub directory: Option<String>,

    #[clap(
        short = 's',
        long = "source"
    )]
    pub source: Option<String>,
}
