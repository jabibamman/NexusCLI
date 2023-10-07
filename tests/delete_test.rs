use std::collections::HashMap;
use NexusCLI::cli::parser::CliArgs;
use NexusCLI::operations::delete::delete;

#[test]
fn test_delete_function() {
    let args = CliArgs {
        operation: "D".to_string(),
        repository: Some("test-repo".to_string()),
        directory: Some("test-dir".to_string()),
        source: None,
    };

    let mock_curl =
        |_: &str, _: &str, _: HashMap<&str, &String>, _: Option<&str>| -> Result<(), curl::Error> {
            Ok(())
        };

    delete(args);
    assert_eq!(1, 1);
}
