use std::fmt;

/// An error that occurred while parsing a TOML document.
/// Mimics `TOMLDecodeError` from the Python tomli library.
#[derive(Debug, PartialEq, Clone)]
pub struct TomlError {
    message: String,
    pos: Option<usize>,
    line: Option<usize>,
    col: Option<usize>,
}

impl TomlError {
    /// Creates a generic error without specific positional information.
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            pos: None,
            line: None,
            col: None,
        }
    }

    /// Creates an error at a specific byte position, dynamically calculating
    /// the line and column numbers identical to Python tomli's logic.
    pub fn at_pos<S: Into<String>>(message: S, pos: usize, src: &str) -> Self {
        let (line, col) = Self::calculate_line_col(src, pos);
        Self {
            message: message.into(),
            pos: Some(pos),
            line: Some(line),
            col: Some(col),
        }
    }

    /// Calculates 1-indexed line and column numbers.
    fn calculate_line_col(src: &str, pos: usize) -> (usize, usize) {
        let safe_pos = std::cmp::min(pos, src.len());
        let prefix = &src[..safe_pos];

        let line = prefix.chars().filter(|&c| c == '\n').count() + 1;
        let col = if let Some(last_newline) = prefix.rfind('\n') {
            prefix[last_newline + 1..].chars().count() + 1
        } else {
            prefix.chars().count() + 1
        };

        (line, col)
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.col) {
            (Some(line), Some(col)) => {
                write!(f, "{} (at line {}, column {})", self.message, line, col)
            }
            _ => {
                if let Some(pos) = self.pos {
                    write!(f, "{} (at byte offset {})", self.message, pos)
                } else {
                    write!(f, "{}", self.message)
                }
            }
        }
    }
}

impl std::error::Error for TomlError {}
