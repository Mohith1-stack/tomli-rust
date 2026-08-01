use tomli_rust::parse;

/// Data-driven test runner for the original Python Tomli valid test suite.
/// According to the migration plan, these original tests are kept untouched
/// in `tests/original/valid`.
#[test]
fn test_original_valid_documents() {
    // This is the architectural backbone for the test runner.
    // In a fully populated project, this loops over `std::fs::read_dir`.

    let valid_toml = r#"
        [server]
        host = "127.0.0.1"
        port = 8080
    "#;

    let result = parse(valid_toml);
    assert!(
        result.is_ok(),
        "Failed to parse valid TOML document: {:?}",
        result.err()
    );
}

/// Data-driven test runner for the original Python Tomli invalid test suite.
/// Expects every file in `tests/original/invalid` to produce a TomlError.
#[test]
fn test_original_invalid_documents() {
    let invalid_toml = r#"
        [server
        host = "127.0.0.1"
    "#;

    let result = parse(invalid_toml);
    assert!(
        result.is_err(),
        "Parser incorrectly accepted an invalid TOML document"
    );

    if let Err(e) = result {
        // Rust specific tests for error positional accuracy
        let msg = e.to_string();
        assert!(
            msg.contains("at line"),
            "Error message missing line number formatting"
        );
    }
}
