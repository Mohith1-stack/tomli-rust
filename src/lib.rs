pub mod datetime;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod tokenizer;
pub mod value;

pub use datetime::{Date, Datetime, Offset, Time};
pub use error::TomlError;
pub use value::{Table, Value};

/// Parses a TOML document from a string.
/// This corresponds to `tomli.loads()` in Python.
///
/// # Errors
/// Returns a `TomlError` if the string contains invalid TOML syntax.
pub fn parse(src: &str) -> Result<Table, TomlError> {
    let parser = parser::Parser::new(src);
    parser.parse()
}

/// Parses a TOML document from a byte slice.
/// This behaves similarly to `tomli.load()`, accepting raw bytes and verifying UTF-8.
///
/// # Errors
/// Returns a `TomlError` if the bytes are not valid UTF-8 or contain invalid TOML syntax.
pub fn parse_bytes(src: &[u8]) -> Result<Table, TomlError> {
    let s = std::str::from_utf8(src).map_err(|_| TomlError::new("Input must be valid UTF-8"))?;
    parse(s)
}
