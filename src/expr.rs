use crate::literal::Literal as Object;
use crate::token::Token;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Binary {
    pub fn new(
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>) -> Self {
        Binary { left, operator, right }
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}
impl Grouping {
    pub fn new(
        expression: Box<Expr>) -> Self {
        Grouping { expression }
    }
}

pub struct Literal {
    pub value: Object,
}
impl Literal {
    pub fn new(
        value: Object) -> Self {
        Literal { value }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Unary {
    pub fn new(
        operator: Token,
        right: Box<Expr>) -> Self {
        Unary { operator, right }
    }
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
}
