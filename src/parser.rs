use crate::token::Token;
use crate::token_types::TokenType;
use crate::ast_printer::*;
use crate::literal::Literal;
use crate::error::ParseError;
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


// shortcuts for building nodes, so the parser doesn't spell out the struct and Box::new each time.
// they take Expr instead of Box<Expr> and do the boxing here
impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    pub fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Unary {
            oeprator: operator,
            right: Box::new(right),
        })
    }
    pub fn grouping(middle: Expr) -> Expr {
        Expr::Grouping(Grouping {
            middle: Box::new(middle),
        })
    }
    pub fn literal(value: Literal) -> Expr {
        Expr::Literal(value)
    }
}

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

    // the entry point: parse the whole token list as one expression
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        return self.expression();
    }

    // implementations of cfg
    // every rule returns a Result, and ? passes an error straight up to parse()
    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.equality();
    }
    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        // either operator continues the chain, so this is ||, not &&
        while self.match_types(TokenType::BangEqual) || self.match_types(TokenType::EqualEqual) {
            // Binary owns its Token, so copy it out of the borrowed slice
            let operator = self.previous().clone();
            let right = self.comparison()?;
            // Expr::binary does the boxing itself
            expr = Expr::binary(expr, operator, right);
        }
        return Ok(expr);
    }
    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_types(TokenType::Greater)
            || self.match_types(TokenType::GreaterEqual)
            || self.match_types(TokenType::Less)
            || self.match_types(TokenType::LessEqual)
        {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }
        return Ok(expr);
    }
    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_types(TokenType::Minus) || self.match_types(TokenType::Plus) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }
        return Ok(expr);
    }
    // factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_types(TokenType::Slash) || self.match_types(TokenType::Star) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right);
        }
        return Ok(expr);
    }
    // unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(TokenType::Bang) || self.match_types(TokenType::Minus) {
            let operator = self.previous().clone();
            // unary calls itself, so !!x and --x nest to the right
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }
        return self.primary();
    }
    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(TokenType::False) {
            return Ok(Expr::literal(Literal::Bool(false)));
        }
        if self.match_types(TokenType::True) {
            return Ok(Expr::literal(Literal::Bool(true)));
        }
        if self.match_types(TokenType::Nil) {
            return Ok(Expr::literal(Literal::Nil));
        }
        if self.match_types(TokenType::Number) || self.match_types(TokenType::Str) {
            // the scanner already turned the text into a value, so reuse it
            return Ok(Expr::literal(self.previous().literal().clone()));
        }
        if self.match_types(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        }
        // nothing above matched, so this token can't start an expression
        return Err(self.error(self.peek(), "Expect expression."));
    }

    // advance past a token that has to be there, or report it missing
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&'a Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        return Err(self.error(self.peek(), message));
    }
    fn error(&self, token: &Token, message: &str) -> ParseError {
        if token.token_type() == TokenType::Eof {
            return ParseError::new(token.line, None, message);
        }
        return ParseError::new(token.line, Some(token.lexeme().to_string()), message);
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

