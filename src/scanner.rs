

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::error::{ScanError, ScanErrorKind};
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


// safe conversion for number, without unwrap
// returns None when the text isn't a Lox number or is too large to fit in an f64
fn safe_convert_str_to_float(t: &str) -> Option<f64> {
    // only accept what Lox allows: digits with at most one dot.
    // parse on its own would also take things like "1e5", "-1", "inf" and "NaN"
    let mut seen_dot = false;
    for byte in t.bytes() {
        if byte == b'.' && !seen_dot {
            seen_dot = true;
        } else if !byte.is_ascii_digit() {
            return None;
        }
    }

    // parse rounds correctly for every input, which adding digits one at a
    // time with number * 10 + digit can't do once numbers get very large
    let number: f64 = match t.parse() {
        Ok(number) => number,
        Err(_) => return None,
    };

    // too large a number parses to infinity instead of failing
    if number.is_finite() {
        return Some(number);
    }
    return None;
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
    // every error found so far, so one run can report all of them
    errors: Vec<ScanError>,
    start: usize,
    current: usize,
    line: usize,
    // byte offset where the current line begins
    line_start: usize,
    // where the token being scanned begins, since strings can span lines
    token_line: usize,
    token_line_start: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let s = source.as_bytes();
        Scanner {
            source,
            source_as_bytes: s,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            line_start: 0,
            token_line: 1,
            token_line_start: 0,
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
    + error() -> record an error at the start of the current token
    ** */
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    fn advance(&mut self) -> char {
        let c = self.peek();
        // current is a byte offset, so move by the character's size in bytes
        self.current += c.len_utf8();
        // the only place lines are counted, so line and column always agree
        if c == '\n' {
            self.line += 1;
            self.line_start = self.current;
        }
        return c;
    }
    fn add_token(&mut self, token_t: TokenType, lit: Literal) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_t, text.to_owned(), lit, self.token_line));
    }
    fn error(&mut self, kind: ScanErrorKind) {
        // columns count characters, so non-ASCII text doesn't shift them
        let column = self.source[self.token_line_start..self.start].chars().count() + 1;
        self.errors.push(ScanError::new(kind, self.token_line, column));
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
        // ascii is one byte, so keep the fast path for almost all source
        let byte = self.source_as_bytes[self.current];
        if byte.is_ascii() {
            return byte as char;
        }
        // anything else takes multiple bytes, so decode the whole character
        return self.source[self.current..].chars().next().unwrap_or('\0');
    }
    fn handle_string_literal(&mut self) {
        // advance() counts any new lines inside the string
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        // if is_at_end() then non terminated string
        if self.is_at_end() {
            self.error(ScanErrorKind::UnterminatedString);
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
        match safe_convert_str_to_float(new_num) {
            Some(number) => self.add_token(TokenType::Number, Literal::Number(number)),
            None => self.error(ScanErrorKind::NumberTooLarge),
        }
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
            let literal = match ty {
                TokenType::True => Literal::Bool(true),
                TokenType::False => Literal::Bool(false),
                _ => Literal::Nil,
            };
            self.add_token(ty, literal);
        } else {
            self.add_token(TokenType::Identifier, Literal::Nil);
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
            ' ' | '\r' | '\t' | '\n' => {
                // ignore all type of whitespace, advance() already counted the new line
            }
            // otherwise errors
            '"' => {
                self.handle_string_literal();
            }
            _ => {
                // first case, if it si a diit or not
                if self.is_digit(c) {
                    self.handle_number();
                } else if self.is_alpha(c) {
                    self.handle_identifer();
                } 
                else {
                    self.error(ScanErrorKind::UnexpectedCharacter(c));
                }
            }
        }
    }
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScanError>> {
        // while not at end then we scan the tokens
        while !Scanner::is_at_end(self) {
            self.start = self.current;
            self.token_line = self.line;
            self.token_line_start = self.line_start;
            self.scan_token();
        }
        // any error means the tokens can't be trusted
        if !self.errors.is_empty() {
            return Err(std::mem::take(&mut self.errors));
        }
        // otherwise mark endline
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            Literal::Nil,
            self.line,
        ));
        // return a different value for self.tokens
        return Ok(std::mem::take(&mut self.tokens));
    }
}

#[cfg(test)]
#[path = "../tests/scanner_tests.rs"]
mod tests;
