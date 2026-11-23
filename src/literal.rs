// --------------------------------------------------------
//       LITERAL
// --------------------------------------------------------
// literal is defined based off the kind of literal that exists in Lox

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(s) => f.pad(s),
            Literal::Number(n) => fmt::Display::fmt(n, f),
            Literal::Bool(b) => fmt::Display::fmt(b, f),
            Literal::Nil => f.pad("nil"),
        }
    }
}