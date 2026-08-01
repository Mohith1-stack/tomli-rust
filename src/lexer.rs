pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from a TOML string.
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    /// Returns the original source string.
    pub fn src(&self) -> &'a str {
        self.src
    }

    /// Returns the current byte position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns true if the lexer has reached the end of the input.
    pub fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    /// Peeks at the next character without consuming it.
    pub fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    /// Consumes and returns the next character.
    pub fn next_char(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// Peeks at the next `n` bytes as a string slice.
    pub fn peek_str(&self, n: usize) -> Option<&'a str> {
        self.src.get(self.pos..self.pos + n)
    }

    /// Checks if the remaining source starts with a specific prefix.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.src[self.pos..].starts_with(prefix)
    }

    /// Advances the lexer by `n` bytes.
    pub fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    /// Skips spaces and tabs (`\t`, ` `).
    pub fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    /// Skips spaces, tabs, and newlines.
    pub fn skip_ws_and_newline(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\n' {
                self.advance(1);
            } else if c == '\r' && self.starts_with("\r\n") {
                self.advance(2);
            } else {
                break;
            }
        }
    }

    /// Skips a TOML comment starting with `#`.
    pub fn skip_comment(&mut self) -> bool {
        if self.peek() == Some('#') {
            while let Some(c) = self.peek() {
                if c == '\n' {
                    break;
                }
                // TOML specifies that control characters are not permitted in comments.
                // We'll let the Tokenizer/Parser handle strict validation or we can validate here.
                self.next_char();
            }
            true
        } else {
            false
        }
    }

    /// Consumes all consecutive whitespace and comments.
    pub fn skip_comments_and_ws(&mut self) {
        loop {
            let start = self.pos;
            self.skip_ws_and_newline();
            self.skip_comment();
            if self.pos == start {
                break;
            }
        }
    }

    /// Returns a slice of the source from `start` to the current position.
    pub fn slice_from(&self, start: usize) -> &'a str {
        &self.src[start..self.pos]
    }
}
