use crate::literal::Literal;
use crate::token_types::TokenType;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]

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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}