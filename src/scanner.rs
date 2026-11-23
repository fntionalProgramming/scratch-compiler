use crate::Lox;
use crate::token::Literal;
use crate::token::Token;
use crate::token_types::TokenType;
use std::str::Chars;

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
    + match char, matches the character against the token type
    + peek() -> trivial
    + handle_str_literal() -> handle when there is a string literal
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
    fn match_char(&mut self, expected: char) -> bool {
        // base cases
        // if is at end then no characters to match -> false
        // if not match as expected. -> false
        if self.is_at_end() {
            return false;
        }
        if self.source_as_chars.nth(self.current as usize).unwrap() != expected {
            return false;
        }
        // increment the cuirrent index after
        self.current += 1;
        return true;
    }
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source_as_chars.nth(self.current as usize).unwrap();
    }
    fn handle_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {

        }
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
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, Literal::Nil);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, Literal::Nil);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, Literal::Nil);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, Literal::Nil);
            }
            // handle longer lexemes
            // specifically when we encounter an lexmes for comments
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Nil);
                }
            }
            ' ' | '\r' | '\t' => {
                // ignore all type of whitespace
            }
            '\n' => self.line += 1,
            // otherwise errors
            "\"" => {
                self.handle_str_literal();
            }
            _ => {
                println!("This will be changed later so it doesnt matter")
            }
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
