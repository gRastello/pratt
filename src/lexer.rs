use crate::token::*;

pub struct Lexer {
    /// Source code.
    source: String,
    /// Start of lexeme.
    start: usize,
    /// Current character.
    current: usize,
    /// Scanned tokens.
    tokens: Vec<Token>,
}

impl Lexer {
    /// Create a new lexer.
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }

    /// Return true if we're at the end of the scannable source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Advance the lexer one character.
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    /// Peek the next character (does not advance the lexer).
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    /// Store a  new token  in the lexer.
    fn add_token(&mut self, kind: TokenKind) {
        // Get lexeme.
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        self.tokens.push(Token::new(kind, lexeme));
    }

    /// Scan tokens.
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScanError>> {
        let mut errors = Vec::new();

        while !self.is_at_end() {
            // We are at the start of a new lexeme.
            self.start = self.current;
            if let Err(mut errs) = self.scan_token() {
                errors.append(&mut errs)
            }
        }

        // Push the Eof token.
        self.tokens.push(Token::new(TokenKind::Eof, String::new()));

        // Return errors, if any, or the tokens.
        if errors.is_empty() {
            Ok(self.tokens.clone())
        } else {
            Err(errors)
        }
    }

    /// Scan a single token.
    fn scan_token(&mut self) -> Result<(), Vec<ScanError>> {
        let mut errors = Vec::new();

        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' => (),
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            '*' => self.add_token(TokenKind::Star),
            '/' => self.add_token(TokenKind::Slash),
            '^' => self.add_token(TokenKind::Cap),
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            _ => {
                if c.is_digit(10) {
                    if let Err(err) = self.number() {
                        errors.push(err);
                    }
                } else {
                    errors.push(ScanError::new("unrecognized token", self.start + 1))
                }
            }
        };

        // Return errors, if any, or the token.
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Scan a number token. We allow real numbers but not in forms such as .23
    /// or 23. (we want 0.23 and 23.0 or just 23).
    fn number(&mut self) -> Result<(), ScanError> {
        // Consume all digits befor an eventual dot.
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a decimal part.
        if self.peek() == '.' {
            self.advance();

            if self.peek().is_digit(10) {
                while self.peek().is_digit(10) {
                    self.advance();
                }
            } else {
                return Err(ScanError::new("misformed number", self.start + 1));
            }
        }

        // Add the token.
        let number = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenKind::Number(number));

        Ok(())
    }
}

/// A lexing error.
pub struct ScanError {
    /// A description of the error.
    pub description: String,
    /// The position (column) of the error.
    pub position: usize,
}

impl ScanError {
    /// Make a new lexing error.
    fn new(description: &str, position: usize) -> Self {
        ScanError {
            description: description.to_string(),
            position,
        }
    }
}
