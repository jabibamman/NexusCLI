pub struct CliArgs {
    pub config: Option<String>
/*pub operation: Operation,*/
}

pub fn parse_args() -> CliArgs {

    CliArgs {
        config: None
    }
}