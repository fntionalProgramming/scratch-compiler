use crate::Lox;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_types::TokenType;

// --------------------------------------------------------
//       Scanner
// --------------------------------------------------------
// scanner effectively scan the code, parse it then potentiall converts it to a list of tokens
pub struct Scanner<'a> {
    // the source text that is to be scan
    source: &'a str,
    // the vector of token that the scanner will converts the source into
    tokens: Vec<Token>,
    // for keeping track of where the scanner is in the file
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
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
        return self.source[self.current..].chars().next().unwrap();
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
    fn handle_number(&mut self) {
        // in here we consider both digits and fractional numbers
        while self.is_digit(self.peek()) {
            self.advance();
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
