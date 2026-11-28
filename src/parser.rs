use crate::token::{self, Token};
use crate::token_types::{self, TokenType};
use crate::ast_printer::*;
// -----------------------------------------
//             PARSER 
// -----------------------------------------
// list of cfgs
// also we define the precedent to be
// EQUALITY -> COMPARISION -> ADDITION -> MULTIPLICATION -> UNARY
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

struct Parser<'a> {
    tokens: &'a [Token],
    current: usize
} 
impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { 
            tokens, 
            current: 0 
        }
    }

    // implementations of cfg
    fn expression(&mut self) -> Expr {
        return self.equality();
    }
    fn equality(&mut self) -> Expr {
        // expr for comparision
        let mut expr = self.comparision();
        // either operator continues the chain, so this is ||, not &&
        while self.match_types(TokenType::BangEqual) || self.match_types(TokenType::EqualEqual) {
            // Binary owns its Token, so copy it out of the borrowed slice
            let operator = self.previous().clone();
            let right = self.comparision();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }
        return expr
    }
    // placeholder so equality compiles; replace with the real rule
    fn comparision(&mut self) -> Expr {
        todo!("comparision")
    }
    fn term() {
    
    }
    fn factor() {
        
    }
    fn unary() {

    }
    fn primary() {

    }
    // functions aside from the CFG implementations
    // match for one types only
    fn match_types(&mut self, token_type: TokenType) -> bool {
        // if check return true then we advance one token
        if self.check(token_type) {
            self.advance();
            return true;
        }
        return false;
    }
    fn check(&self, token_type: TokenType) -> bool {
        // 2 check if the current type match the token type
        if self.is_at_end() {
            return false
        }
        return self.peek().token_type() == token_type;
    }
    // peek, peeks in the current token, not peek at the next token
    // returns a reference with the slice's lifetime 'a, not the parser's,
    fn peek(&self) -> &'a Token {
        return &self.tokens[self.current];
    }
    fn is_at_end(&self) -> bool {
        return self.peek().token_type() == TokenType::Eof;
    }
    fn advance(&mut self) -> &'a Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    // unwrap is safe since previous is always valid
    fn previous(&self) -> &'a Token {
        return unsafe {
            self.tokens.get_unchecked(self.current - 1)
        }
    }
}

