use crate::literal::Literal;
use crate::token_types::TokenType;
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;



#[derive(Debug, Clone, PartialEq)]

// --------------------------------------------------------
//       TOKEN
// --------------------------------------------------------

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    pub(crate) line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token { 
            token_type, 
            lexeme, 
            literal: literal , 
            line: line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}