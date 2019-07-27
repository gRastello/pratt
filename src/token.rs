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

    /// Return true if the Token is an operator.
    pub fn is_operator(&self) -> bool {
        match self.kind {
            TokenKind::Minus
            | TokenKind::Plus
            | TokenKind::Slash
            | TokenKind::Star
            | TokenKind::Cap => true,
            _ => false,
        }
    }

    /// Return true if the Token is a number.
    pub fn is_number(&self) -> bool {
        match self.kind {
            TokenKind::Number(_) => true,
            _ => false,
        }
    }

    /// Return the binding power of an operator.
    pub fn binding_power(&self) -> u8 {
        match self.kind {
            TokenKind::Minus | TokenKind::Plus => 10,
            TokenKind::Star | TokenKind::Slash => 20,
            TokenKind::Cap => 30,
            _ => panic!("Numbers can't have binding power!"),
        }
    }

    /// Return true if the operator is left-associative.
    pub fn is_left_associative(&self) -> bool {
        match self.kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Star => true,
            TokenKind::Cap => false,
            _ => panic!("NUmbers can't be left (or right) associative!"),
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
