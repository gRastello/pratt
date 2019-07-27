#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    /// Create a new token from kind and lexeme.
    pub fn new(kind: TokenKind, lexeme: String) -> Self {
        Token { kind, lexeme }
    }

    /// Return true if the Token is a number.
    pub fn is_number(&self) -> bool {
        match self.kind {
            TokenKind::Number(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    /// A number.
    Number(f64),
    /// Sum operator.
    Plus,
    /// Minus operator
    Minus,
    /// Multiplication operator.
    Star,
    /// Division operator.
    Slash,
    /// Exponentiation operator.
    Cap,
    /// EOF token.
    Eof,
}
