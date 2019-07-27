use crate::expr::*;
use crate::token::*;

/// The parser.
pub struct Parser {
    /// Tokens to parse.
    tokens: Vec<Token>,
    /// Index of current token.
    current: usize,
    // Left parselets (led).
    // consequent_parselets: Vec<Box<ConsequentParselet>>,
}

impl Parser {
    /// Make a new parser.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Dispatch a consequent parselet.
    fn get_consequent_parselet(&self, token: &Token) -> Result<Box<ConsequentParselet>, ParseError> {
        match token.kind {
            TokenKind::Plus => Ok(Box::new(OperatorParselet::new(TokenKind::Plus))),
            TokenKind::Minus => Ok(Box::new(OperatorParselet::new(TokenKind::Minus))),
            TokenKind::Star => Ok(Box::new(OperatorParselet::new(TokenKind::Star))),
            TokenKind::Slash => Ok(Box::new(OperatorParselet::new(TokenKind::Slash))),
            TokenKind::Cap => Ok(Box::new(OperatorParselet::new(TokenKind::Cap))),
            _ => Err(ParseError::new(
                "no parselet found for this token (maybe this is not an operator?)",
                token,
            )),
        }
    }

    /// Advance the parser retrieving a new token.
    fn advance(&mut self) -> &Token {
        let token = self.tokens.get(self.current).unwrap();
        self.current += 1;

        token
    }

    /// Peek the next token without advancing the parser.
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    /// Parse the tokens into an expression.
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.actually_parse(0)
    }

    /// The actual parsing function.
    fn actually_parse(&mut self, current_binding_power: u8) -> Result<Expr, ParseError> {
        // Check that the first token is a number.
        let first_token = self.advance();
        if !first_token.is_number() {
            return Err(ParseError::new("expected number", first_token));
        }
        let mut left_expr = Expr::Literal(Literal::new(first_token.clone()));

        loop {
            let current_token = self.peek().clone();
            if let TokenKind::Eof = current_token.kind {
                break;
            }

            let consequent_parselet = self.get_consequent_parselet(&current_token)?;
            if consequent_parselet.binding_power() <= current_binding_power {
                break;
            }

            self.advance();
            left_expr = consequent_parselet.parse(self, current_token, left_expr)?;
        }

        Ok(left_expr)
    }
}

/// Trait for consequent parselets.
trait ConsequentParselet {
    /// Return the operator's binding power.
    fn binding_power(&self) -> u8;
    /// The parse function.
    fn parse(
        &self,
        parser: &mut Parser,
        current_token: Token,
        left_expr: Expr,
    ) -> Result<Expr, ParseError>;
}

/// Operator parselet.
struct OperatorParselet {
    /// Binding power of the operator.
    binding_power: u8,
    /// True if the operator is left associative.
    left_associative: bool,
}

impl OperatorParselet {
    /// Make a new operator parselet from an operator.
    fn new(operator: TokenKind) -> Self {
        match operator {
            TokenKind::Plus | TokenKind::Minus => OperatorParselet {
                binding_power: 10,
                left_associative: true,
            },
            TokenKind::Star | TokenKind::Slash => OperatorParselet {
                binding_power: 20,
                left_associative: true,
            },
            TokenKind::Cap => OperatorParselet {
                binding_power: 30,
                left_associative: false,
            },
            _ => panic!("Can't make an operator parselet from a non-operator"),
        }
    }
}

impl ConsequentParselet for OperatorParselet {
    fn binding_power(&self) -> u8 {
        self.binding_power
    }

    fn parse(
        &self,
        parser: &mut Parser,
        current_token: Token,
        left_expr: Expr,
    ) -> Result<Expr, ParseError> {
        let binding_power = if self.left_associative {
            self.binding_power
        } else {
            self.binding_power - 1
        };
        let right_expr = parser.actually_parse(binding_power)?;

        Ok(Expr::Binary(Binary::new(
            current_token,
            left_expr,
            right_expr,
        )))
    }
}

/// A parse error.
pub struct ParseError {
    pub description: String,
    pub token: Token,
}

impl ParseError {
    /// Make a new parse error.
    fn new(description: &str, token: &Token) -> Self {
        ParseError {
            description: description.to_string(),
            token: token.clone(),
        }
    }
}
