use crate::expr::*;
use crate::token::*;

/// The parser.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Make a new parser.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
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
    fn actually_parse(&mut self, last_operator_binding_power: u8) -> Result<Expr, ParseError> {
        // Check that the first token is a number.
        let first_token = self.advance();
        if !first_token.is_number() {
            return Err(ParseError::new("expected number", first_token));
        }
        let mut left_expr = Expr::Literal(Literal::new(first_token.clone()));

        loop {
            let next_token = self.peek();

            // If the next token is an operator and binds strongly than the
            // previous one parse after it otherwise exit the loop.
            //
            // We grow the expression to the right as far as we can find
            // operators that bind at least as strongly as the previus one and
            // we grow the expression to the left (by exiting the loop thus
            // "lifting") when this is impossible.
            if next_token.is_operator() {
                if next_token.binding_power() > last_operator_binding_power {
                    let operator = self.advance().clone();

                    // If the operator is right associative we reduce its
                    // binding power by 1 so that we keep recursing if we
                    // hit the same operator again.
                    let operator_binding_power = if operator.is_left_associative() {
                        operator.binding_power()
                    } else {
                        operator.binding_power() - 1
                    };
                    let rigth_expr = self.actually_parse(operator_binding_power)?;
                    
                    left_expr = Expr::Binary(Binary::new(operator, left_expr, rigth_expr));
                } else {
                    break;
                }
            } else if let TokenKind::Eof = next_token.kind {
                break;
            } else {
                return Err(ParseError::new("expected operator", next_token));
            }
        }

        Ok(left_expr)
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
