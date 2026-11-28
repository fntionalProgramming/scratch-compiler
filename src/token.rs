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

    // the exact text from the source, like "-" or "123"
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    // TokenType is Copy, so this hands back a copy instead of a reference
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
#[path = "../tests/token_tests.rs"]
mod tests;
