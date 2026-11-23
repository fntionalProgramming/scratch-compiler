use core::fmt;

use crate::token_types::TokenType;
#[derive(Debug, Clone, PartialEq)]

// --------------------------------------------------------
//       LITERAL
// --------------------------------------------------------
// literal is defined based off the kind of literal that exists in Lox

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


// --------------------------------------------------------
//       TOKEN
// --------------------------------------------------------

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    pub(crate) line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Self {
        Token { 
            token_type, 
            lexeme, 
            literal: literal , 
            line: line
        }
    }
    pub fn to_string(&self) -> String{
        return format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}