use crate::literal::Literal as Object;
use crate::token::Token;

struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Binary {
    fn new(
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>) -> Self {
        Binary { left, operator, right }
    }
}

struct Grouping {
    pub expression: Box<Expr>,
}
impl Grouping {
    fn new(
        expression: Box<Expr>) -> Self {
        Grouping { expression }
    }
}

struct Literal {
    pub value: Object,
}
impl Literal {
    fn new(
        value: Object) -> Self {
        Literal { value }
    }
}

struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Unary {
    fn new(
        operator: Token,
        right: Box<Expr>) -> Self {
        Unary { operator, right }
    }
}

enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
}
