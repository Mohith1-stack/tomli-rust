use std::collections::HashSet;

use crate::error::TomlError;
use crate::lexer::Lexer;
use crate::tokenizer::{Token, Tokenizer};
use crate::value::{Table, Value};

/// Tracks namespace immutability as defined in Python `tomli`'s `Flags` class.
pub struct Flags {
    frozen: HashSet<Vec<String>>,
    explicit_nests: HashSet<Vec<String>>,
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            frozen: HashSet::new(),
            explicit_nests: HashSet::new(),
        }
    }
}

impl Flags {

    pub fn is_frozen(&self, key: &[String]) -> bool {
        self.frozen.contains(key)
    }

    pub fn set_explicit_nest(&mut self, key: Vec<String>) {
        self.explicit_nests.insert(key);
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    flags: Flags,
    out: Table,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(Lexer::new(src)),
            flags: Flags::new(),
            out: Table::new(),
        }
    }

    /// The core `loads` loop mapping directly from Python `tomli`.
    pub fn parse(mut self) -> Result<Table, TomlError> {
        let mut current_header = Vec::new();

        loop {
            self.tokenizer.lexer_mut().skip_ws();

            if self.tokenizer.lexer_mut().is_eof() {
                break;
            }

            let c = match self.tokenizer.lexer_mut().peek() {
                Some(ch) => ch,
                None => break,
            };

            if c == '\n' || c == '\r' {
                self.tokenizer.lexer_mut().next_char();
                continue;
            }

            if c == '#' {
                self.tokenizer.lexer_mut().skip_comment();
                continue;
            }

            // Dispatch to syntax rules just like Python Tomli
            if c == '[' {
                current_header = self.parse_table_header()?;
            } else if c.is_ascii_alphanumeric() || c == '"' || c == '\'' || c == '_' || c == '-' {
                self.parse_key_value_rule(&current_header)?;
            } else {
                let pos = self.tokenizer.lexer_mut().pos();
                let src = self.tokenizer.lexer_mut().src();
                return Err(TomlError::at_pos("Invalid statement", pos, src));
            }
        }

        Ok(self.out)
    }

    /// Emulates `create_dict_rule` and `create_list_rule` in Python.
    fn parse_table_header(&mut self) -> Result<Vec<String>, TomlError> {
        let is_array = self.tokenizer.lexer_mut().starts_with("[[");
        let start_pos = self.tokenizer.lexer_mut().pos();
        let src = self.tokenizer.lexer_mut().src();

        if is_array {
            self.tokenizer.lexer_mut().advance(2);
        } else {
            self.tokenizer.lexer_mut().advance(1);
        }

        self.tokenizer.lexer_mut().skip_ws();

        // Parse the key... (stubbed for brevity)
        let key_str = match self.tokenizer.parse_bare_key() {
            Some(Token::BareKey(k)) => k.to_string(),
            _ => {
                return Err(TomlError::at_pos(
                    "Expected valid table key",
                    self.tokenizer.lexer_mut().pos(),
                    src,
                ))
            }
        };

        let key = vec![key_str];

        // Validate immutability
        if self.flags.is_frozen(&key) {
            return Err(TomlError::at_pos(
                "Cannot mutate immutable namespace",
                start_pos,
                src,
            ));
        }

        self.flags.set_explicit_nest(key.clone());

        self.tokenizer.lexer_mut().skip_ws();
        let expected_close = if is_array { "]]" } else { "]" };
        if !self.tokenizer.lexer_mut().starts_with(expected_close) {
            return Err(TomlError::at_pos("Expected closing bracket for table header", self.tokenizer.lexer_mut().pos(), src));
        }
        self.tokenizer.lexer_mut().advance(expected_close.len());

        // Skip to end of line to conclude stub...
        while let Some(c) = self.tokenizer.lexer_mut().next_char() {
            if c == '\n' {
                break;
            }
        }

        Ok(key)
    }

    /// Emulates `key_value_rule` from Python.
    fn parse_key_value_rule(&mut self, _header: &[String]) -> Result<(), TomlError> {
        let src = self.tokenizer.lexer_mut().src();
        let start_pos = self.tokenizer.lexer_mut().pos();

        let key_str = match self.tokenizer.parse_bare_key() {
            Some(Token::BareKey(k)) => k.to_string(),
            _ => return Err(TomlError::at_pos("Expected valid key", start_pos, src)),
        };

        self.tokenizer.lexer_mut().skip_ws();

        if !self.tokenizer.consume_char('=') {
            return Err(TomlError::at_pos(
                "Expected '=' after key",
                self.tokenizer.lexer_mut().pos(),
                src,
            ));
        }

        self.tokenizer.lexer_mut().skip_ws();

        // In a full implementation, `parse_value` would recursively build arrays/tables
        // We'll skip to end of line for the structural backbone.
        while let Some(c) = self.tokenizer.lexer_mut().next_char() {
            if c == '\n' {
                break;
            }
        }

        // Add to our NestedDict analogue `out`
        self.out
            .insert(key_str, Value::String("parsed_value".to_string()));

        Ok(())
    }
}
