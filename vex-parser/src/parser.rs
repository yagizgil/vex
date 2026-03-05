use crate::preparser::PreParser;
use vex_core::token::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut preparser = PreParser::new(tokens);
        let refined = preparser.process();
        Self {
            tokens: refined,
            idx: 0,
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.idx.min(self.tokens.len() - 1)]
    }

    pub fn advance(&mut self) -> Token {
        let t = self.tokens[self.idx.min(self.tokens.len() - 1)].clone();
        if self.idx < self.tokens.len() && !matches!(t.kind, TokenType::Eof) {
            self.idx += 1;
        }
        t
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenType::Eof)
    }
}
