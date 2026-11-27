use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};

// empty struct to be implement on
pub struct AstPrinter {}

impl AstPrinter {
    // main function print
    pub fn print(&mut self, expr: &Expr) -> String {
        // instead of running an accept chain, we wish to do pattern matching for visitng
        // Expr is an enum, so pick the visit method from the variant
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
        }
    }

    // wrap name in parathensize for the ease of printing
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&self.print(expr));
        }
        result.push(')');
        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        self.parenthesize(expr.operator.lexeme(), &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        // Object's Display already prints nil as "nil"
        expr.value.to_string()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme(), &[&expr.right])
    }
}
