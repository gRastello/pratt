use crate::expr::*;

pub struct PrettyPrinter {
    pub pretty_text: String,
}

impl PrettyPrinter {
    /// Make a new, empty, pretty printer.
    pub fn new() -> Self {
        PrettyPrinter {
            pretty_text: String::new(),
        }
    }
}

impl ExprVisitor for PrettyPrinter {
    fn visit_binary(&mut self, expr: &Binary) {
        self.pretty_text.push('(');
        self.pretty_text.push_str(&expr.operator.lexeme);
        self.pretty_text.push(' ');
        walk_expr(self, &expr.left);
        self.pretty_text.push(' ');
        walk_expr(self, &expr.right);
        self.pretty_text.push(')');
    }

    fn visit_literal(&mut self, expr: &Literal) {
        self.pretty_text.push_str(&expr.token.lexeme);
    }
}
