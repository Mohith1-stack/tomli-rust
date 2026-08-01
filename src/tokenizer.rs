use crate::lexer::Lexer;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Equals,                 // =
    LBracket,               // [
    RBracket,               // ]
    DoubleLBracket,         // [[
    DoubleRBracket,         // ]]
    LBrace,                 // {
    RBrace,                 // }
    Comma,                  // ,
    Dot,                    // .
    BareKey(&'a str),       // Bare key
    BasicString(&'a str),   // "..."
    LiteralString(&'a str), // '...'
}

pub struct Tokenizer<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn lexer_mut(&mut self) -> &mut Lexer<'a> {
        &mut self.lexer
    }

    /// Peeks the next punctuation character.
    pub fn peek_punct(&mut self) -> Option<char> {
        self.lexer.skip_ws();
        self.lexer.peek()
    }

    /// Parses a bare key part according to BARE_KEY_CHARS: [A-Za-z0-9_-]
    pub fn parse_bare_key(&mut self) -> Option<Token<'a>> {
        let start = self.lexer.pos();
        while let Some(c) = self.lexer.peek() {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                self.lexer.next_char();
            } else {
                break;
            }
        }
        let pos = self.lexer.pos();
        if pos > start {
            Some(Token::BareKey(self.lexer.slice_from(start)))
        } else {
            None
        }
    }

    /// Consumes an expected token or returns false.
    pub fn consume_char(&mut self, expected: char) -> bool {
        if self.lexer.peek() == Some(expected) {
            self.lexer.next_char();
            true
        } else {
            false
        }
    }

    /// Parses a literal string (single quotes).
    pub fn parse_literal_string(&mut self) -> Result<Token<'a>, &'static str> {
        self.lexer.next_char(); // skip '
        let start = self.lexer.pos();
        while let Some(c) = self.lexer.peek() {
            if c == '\'' {
                let s = self.lexer.slice_from(start);
                self.lexer.next_char(); // skip closing '
                return Ok(Token::LiteralString(s));
            }
            if c == '\n' || c < '\x08' {
                // Illegal control chars or unescaped newlines in non-multiline string
                return Err("Invalid character in literal string");
            }
            self.lexer.next_char();
        }
        Err("Unterminated literal string")
    }

    /// Parses a basic string (double quotes).
    /// Note: Full unescaping is usually handled during AST generation,
    /// but the tokenizer groups the span.
    pub fn parse_basic_string(&mut self) -> Result<Token<'a>, &'static str> {
        self.lexer.next_char(); // skip "
        let start = self.lexer.pos();
        let mut escaped = false;

        while let Some(c) = self.lexer.peek() {
            if escaped {
                escaped = false;
                self.lexer.next_char();
                continue;
            }

            if c == '\\' {
                escaped = true;
                self.lexer.next_char();
                continue;
            }

            if c == '"' {
                let s = self.lexer.slice_from(start);
                self.lexer.next_char(); // skip closing "
                return Ok(Token::BasicString(s));
            }

            if c == '\n' || c < '\x08' {
                return Err("Invalid character in basic string");
            }

            self.lexer.next_char();
        }
        Err("Unterminated basic string")
    }
}
