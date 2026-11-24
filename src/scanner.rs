

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::Lox;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_types::TokenType;

// --------------------------------------------------------
//       KEYWORDS
// --------------------------------------------------------

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    use TokenType::*;
    HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fun", Fun),
        ("if", If),
        ("nil", Nil),
        ("or", Or),
        ("print", Print),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("var", Var),
        ("while", While),
    ])
});


#[allow(unused)]
// safe conversion for number without unwrap
fn safe_convert_str_to_float(t: &str) -> f64 {
    let mut number: f64 = 0.0;
    let t_bytes = t.as_bytes();
    let mut i: usize = 1;
    // note that if there is no fractional, 
    while i <= t_bytes.len() {
        // second iteration iterating over the decimal parts
        if t_bytes[i] as char == '.' {
            while i <= t_bytes.len() {
                number += ((t_bytes[i - 1] - b'0') * 10_u8.pow((i as u32) + 1)) as f64 / 100.0;
                // increment the index
                i += 1;
            }
            // break when done
            break;
        }
        // bytes conversion
        // and also weird trick for exponentiating
        number += ((t_bytes[i - 1] - b'0') * 10_u8.pow((i as u32) + 1)) as f64;
        // incremental 
        i += 1;
    }
    return number;
}

// --------------------------------------------------------
//       Scanner
// --------------------------------------------------------
// scanner effectively scan the code, parse it then potentiall converts it to a list of tokens
pub struct Scanner<'a> {
    // the source text that is to be scan
    source: &'a str,
    source_as_bytes: &'a [u8],
    // the vector of token that the scanner will converts the source into
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let s = source.as_bytes();
        Scanner {
            source,
            source_as_bytes: s,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
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
    + is_digit() -> check if something is a digit or not
    + peek_next() -> peek only the next character and return the chracter at next
    + is_alpha() -> check if the chracter is alpha numeric or not
    + handle_identifier() -> check if the token is an identifer or not
    + is_alpha_numeric() -> check ifsomething is alpha numeric or not
    ** */
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    fn advance(&mut self) -> char {
        let c = self.peek();
        // current is a byte offset, so move by the character's size in bytes
        self.current += c.len_utf8();
        return c;
    }
    fn add_token(&mut self, token_t: TokenType, lit: Literal) {
        let text: &str = &self.source[self.start..self.current];
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
        if self.peek() != expected {
            return false;
        }
        // increment the cuirrent index after
        self.current += 1;
        return true;
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source_as_bytes[self.current] as char;
    }
    fn handle_string_literal(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            // increase the line when encounter new line
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // if is_at_end() then non terminated string
        if self.is_at_end() {
            lox.error(self.line as i32, "Unterminated string.");
            return;
        }
        self.advance();
        let value = &self.source[(self.start + 1)..(self.current - 1)];
        self.add_token(TokenType::Str, Literal::Str(value.to_string()));
    }
    fn is_digit(&self, c: char) -> bool {
        return c.is_ascii_digit();
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source_as_bytes[self.current + 1] as char;
    }
    fn handle_number(&mut self) {
        // in here we consider both digits and fractional numbers
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume the current fraction
            self.advance();
            // move forward possiblly till the end
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let new_num = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Literal::Number(safe_convert_str_to_float(new_num)));
    }
    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }
    fn is_alphanumeric(&self, c: char) -> bool  {
        return self.is_alpha(c) || self.is_digit(c);
    }
    fn handle_identifer(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        // after tscanning the identifier, we want to know that if it matches anyhting in the list
        let text = &self.source[self.start..self.current];
        if let Some(&ty) = KEYWORDS.get(text) {
            self.add_token(ty, Literal::Nil);
        } else {
            self.add_token(TokenType::Identifier, Literal::Nil);
        }
    }
    fn scan_token(&mut self, lox: &mut Lox) {
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
            '"' => {
                self.handle_string_literal(lox);
            }
            _ => {
                // first case, if it si a diit or not
                if self.is_digit(c) {
                    self.handle_number();
                } else if self.is_alpha(c) {
                    self.handle_identifer();
                } 
                else {
                    lox.error(self.line.try_into().unwrap(), "Unexpected character ");
                }

                println!("This will be changed later so it doesnt matter")
            }
        }
    }
    pub fn scan_tokens(&mut self, lox: &mut Lox) -> Vec<Token> {
        // while not at end then we scan the tokens
        while !Scanner::is_at_end(self) {
            self.start = self.current;
            self.scan_token(lox);
        }
        // otherwise mark endline
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            Literal::Nil,
            self.line,
        ));
        // return a different value for self.tokens
        return std::mem::take(&mut self.tokens);
    }
}
