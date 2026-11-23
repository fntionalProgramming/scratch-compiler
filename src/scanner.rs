use crate::token::Token;

pub struct Scanner {
    source: String,
    token: Vec<Token>,
}

impl Scanner {
    fn new(&mut self, source: String) {
        self.source = source;
    }
}