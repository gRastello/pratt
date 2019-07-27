use crate::expr::*;
use crate::token::*;

/// The parser.
pub struct Parser {
    /// Tokens to parse.
    tokens: Vec<Token>,
    /// Index of current token.
    current: usize,
}

impl Parser {
    /// Make a new parser.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Dispatch an initial parselet.
    fn get_initial_parselet(&self, token: &Token) -> Result<Box<InitialParselet>, ParseError> {
        if token.is_number() {
            Ok(Box::new(NumberParselet::new()))
        } else {
            match token.kind {
                TokenKind::LeftParen => Ok(Box::new(ParenthesisParselet::new())),
                _ => Err(ParseError::new("no parselet found for this token", token)),
            }
        }
    }

    /// Dispatch a consequent parselet.
    fn get_consequent_parselet(&self, token: &Token) -> Option<Box<ConsequentParselet>> {
        match token.kind {
            TokenKind::Plus => Some(Box::new(OperatorParselet::new(TokenKind::Plus))),
            TokenKind::Minus => Some(Box::new(OperatorParselet::new(TokenKind::Minus))),
            TokenKind::Star => Some(Box::new(OperatorParselet::new(TokenKind::Star))),
            TokenKind::Slash => Some(Box::new(OperatorParselet::new(TokenKind::Slash))),
            TokenKind::Cap => Some(Box::new(OperatorParselet::new(TokenKind::Cap))),
            _ => None,
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
        let first_token = self.peek();
        let initial_parselet = self.get_initial_parselet(first_token)?;
        let mut left_expr = initial_parselet.parse(self)?;

        loop {
            let current_token = self.peek().clone();
            if let TokenKind::Eof = current_token.kind {
                break;
            }

            let consequent_parselet = match self.get_consequent_parselet(&current_token) {
                Some(parselet) => parselet,
                None => break,
            };
            if consequent_parselet.binding_power() <= current_binding_power {
                break;
            }

            self.advance();
            left_expr = consequent_parselet.parse(self, current_token, left_expr)?;
        }

        Ok(left_expr)
    }
}

/// Trait for initial (nud) parselets.
trait InitialParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Expr, ParseError>;
}

struct NumberParselet {}

impl NumberParselet {
    /// Make a new number parselet.
    fn new() -> Self {
        NumberParselet {}
    }
}

impl InitialParselet for NumberParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Expr, ParseError> {
        let token = parser.advance();
        Ok(Expr::Literal(Literal::new(token.clone())))
    }
}

struct ParenthesisParselet {}

impl ParenthesisParselet {
    /// Make a new parenthesis parselet.
    fn new() -> Self {
        ParenthesisParselet {}
    }
}

impl InitialParselet for ParenthesisParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Expr, ParseError> {
        // Consume the '('.
        parser.advance();

        let expr = parser.actually_parse(0)?;

        let next_token = parser.peek();
        if let TokenKind::RightParen = next_token.kind {
            parser.advance();
            Ok(Expr::Grouping(Grouping::new(expr)))
        } else {
            Err(ParseError::new("expecting ')'", next_token))
        }
    }
}

/// Trait for consequent (led) parselets.
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
