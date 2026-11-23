use std::str::Chars;

use crate::token::Literal;
use crate::token::Token;
use crate::token_types::TokenType;

// --------------------------------------------------------
//       Scanner
// --------------------------------------------------------
// scanner effectively scan the code, parse it then potentiall converts it to a list of tokens
pub struct Scanner<'a> {
    // the source text that is to be scan
    source: String,
    // for the ase of indexing
    source_as_chars: Chars<'a>,
    // the vector of token that the scanner will converts the source into
    tokens: Vec<Token>,
    // for keeping track of where the scanner is in the file
    start: i32,
    current: i32,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(&mut self, source: &'a String) {
        self.source = source.to_string();
        self.source_as_chars = source.chars();
        self.tokens = Vec::new();
    }

    /* **
    Utilities for scanning tokens
    + scan_tokens() -> use the source string, from either the repl or the scanner then scan the tokens
    + scan_token() -> scan and convert the token
    + is_end() -> check if we are at the end of the file or not

    scan_tokens() is different from scan_token()
    some utilities in scan_token involves
    + advance() -> advances a character
    + add_token(type: TokenType, literal: Literal) -> add a token with a specified literal type
    ** */
    fn is_at_end(&self) -> bool {
        return self.current >= (self.source.len() as i32);
    }
    fn advance(&mut self) -> char {
        let i = self.current + 1;
        // this should never throw an error
        return self.source_as_chars.nth(i as usize).unwrap();
    }
    fn add_token(&mut self, token_t: TokenType, lit: Literal) {
        let text: &str = &self.source[(self.start as usize)..(self.current as usize)];
        self.tokens
            .push(Token::new(token_t, text.to_owned(), lit, self.line));
    }
    fn scan_token(&mut self) {
        // according to crafting intrepreter, this is the real heat of everything
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Nil),
            '}' => self.add_token(TokenType::RightBrace, Literal::Nil),
            ',' => self.add_token(TokenType::Comma, Literal::Nil),
            '.' => self.add_token(TokenType::Dot, Literal::Nil),
            '-' => self.add_token(TokenType::Minus, Literal::Nil),
            '+' => self.add_token(TokenType::Plus, Literal::Nil),
            ';' => self.add_token(TokenType::Semicolon, Literal::Nil),
            '*' => self.add_token(TokenType::Star, Literal::Nil),
            _ => {}
        }
    }
    pub fn scan_tokens(&mut self) {
        // while not at end then we scan the tokens
        while !Scanner::is_at_end(self) {
            self.start = self.current;
            self.scan_token();
        }
        // otherwise mark endline
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            Literal::Nil,
            self.line,
        ));
    }
}
