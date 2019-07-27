use crate::token::*;

/// Visitor for expressions.
pub trait ExprVisitor {
    fn visit_binary(&mut self, expr: &Binary);
    fn visit_literal(&mut self, expr: &Literal);
    fn visit_grouping(&mut self, expt: &Grouping);
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
}

pub fn walk_expr<V: ExprVisitor>(visitor: &mut V, expr: &Expr) {
    match expr {
        Expr::Binary(expr) => visitor.visit_binary(expr),
        Expr::Literal(expr) => visitor.visit_literal(expr),
        Expr::Grouping(expr) => visitor.visit_grouping(expr),
    }
}

/// A binary expression.
#[derive(Debug)]
pub struct Binary {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Binary {
    /// Create a binary expression from an operator and two branches.
    pub fn new(operator: Token, left: Expr, right: Expr) -> Self {
        Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// A literal.
#[derive(Debug)]
pub struct Literal {
    pub token: Token,
}

impl Literal {
    /// Make a new literal from a Token.
    pub fn new(token: Token) -> Self {
        Literal { token }
    }
}

/// A grouping (i.e. an expression between parenthesis).
#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    /// Make a new grouping from an expression.
    pub fn new(expr: Expr) -> Self {
        Grouping {
            expression: Box::new(expr),
        }
    }
}
