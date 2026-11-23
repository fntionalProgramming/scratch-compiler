use core::fmt;
use std::fmt::format;

use crate::token_types::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn std::any::Any>,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Box<dyn std::any::Any>, line: i32) -> Self {
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